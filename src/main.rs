#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod browser;
mod diagnostics;
mod platform;
use boxlinkbox::{
    domain::{Group, Library, Link, normalize_url},
    repository::Repository,
};
use browser::Browser;
use serde::Deserialize;
use serde_json::json;
use tao::{
    dpi::LogicalSize,
    event::{Event as TaoEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    window::WindowBuilder,
};
use wry::{WebContext, WebView, WebViewBuilder};

#[derive(Debug)]
enum Event {
    Command(String),
    Navigation(u64, String),
    Title(u64, String),
    Load(u64, bool, String),
    Popup(String),
}
#[derive(Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
enum Command {
    Ready,
    Legal {
        document: String,
    },
    Quit,
    AddGroup {
        name: String,
    },
    RenameGroup {
        id: u64,
        name: String,
    },
    DeleteGroup {
        id: u64,
    },
    SaveLink {
        group: u64,
        id: Option<u64>,
        title: String,
        url: String,
        note: String,
    },
    DeleteLink {
        group: u64,
        id: u64,
    },
    MoveLink {
        group: u64,
        id: u64,
        direction: i32,
    },
    Undo,
    Theme {
        theme: String,
    },
    OpenGroup {
        id: u64,
    },
    OpenLink {
        group: u64,
        id: u64,
    },
    SelectTab {
        id: u64,
    },
    CloseTab {
        id: u64,
    },
    CloseAll,
    Manage,
    Browse,
    Navigate {
        url: String,
    },
    Back,
    Forward,
    Reload,
    Export,
    Import,
}
fn domain_url(url: &str) -> bool {
    normalize_url(url).is_ok()
}
fn send(ui: &WebView, value: serde_json::Value) {
    // JSON is serialized as a JS string first; user data is never executable code.
    if let Ok(encoded) = serde_json::to_string(&value.to_string()) {
        let _ = ui.evaluate_script(&format!("window.receive(JSON.parse({encoded}))"));
    }
}
fn alert(ui: &WebView, text: &str) {
    send(ui, json!({"kind":"message","text":text}));
}
fn snapshot(ui: &WebView, data: Option<&Library>, browser: &Browser, undo: bool, directory: &str) {
    send(
        ui,
        serde_json::to_value(boxlinkbox::presentation::State {
            kind: "state",
            library: data,
            tabs: browser.tabs.iter().map(|t| &t.info).collect::<Vec<_>>(),
            active: browser.active,
            browsing: browser.visible,
            undo,
            data_directory: directory,
        })
        .unwrap_or(serde_json::Value::Null),
    );
}
fn main() {
    if let Err(message) = run() {
        rfd::MessageDialog::new()
            .set_title("BoxLinkBox を起動できません")
            .set_description(format!(
                "{message}\n\nWebView2 Runtimeが必要です。READMEの案内を確認してください。"
            ))
            .set_level(rfd::MessageLevel::Error)
            .show();
    }
}
fn run() -> Result<(), String> {
    if wry::webview_version().is_err() {
        let open = rfd::MessageDialog::new()
            .set_title("WebView2 Runtimeが必要です")
            .set_description("BoxLinkBoxのアプリ内ブラウザにはMicrosoft Edge WebView2 Runtimeが必要です。Microsoft公式の配布ページを開きますか？\n\nEvergreen Runtimeをインストールしてから、BoxLinkBoxを起動し直してください。")
            .set_buttons(rfd::MessageButtons::YesNo)
            .show();
        if open == rfd::MessageDialogResult::Yes {
            std::process::Command::new("explorer.exe")
                .arg("https://developer.microsoft.com/microsoft-edge/webview2/")
                .spawn()
                .map_err(
                    |_| "Microsoft公式ページを開けませんでした。READMEのURLを確認してください。",
                )?;
        }
        return Ok(());
    }
    let root = platform::data_directory()?;
    std::fs::create_dir_all(&root).map_err(|_| "データ保存先を作成できませんでした。")?;
    let instance = std::fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(root.join("instance.lock"))
        .map_err(|_| "起動用の保存領域を開けませんでした。")?;
    instance.try_lock().map_err(
        |_| "BoxLinkBoxはすでに起動しています。開いているウィンドウを確認してください。",
    )?;
    let mut repo = Repository::open(&root.join("library.db"))?;
    let (mut data, recovered) = match repo.load() {
        Ok(d) => (d, false),
        Err(_) => (repo.recover()?, true),
    };
    if recovered {
        repo.save(&data)?;
    }
    diagnostics::record(
        &root,
        if recovered {
            "startup_recovered"
        } else {
            "startup"
        },
    );
    let mut undo = boxlinkbox::history::History::default();
    let event_loop = EventLoopBuilder::<Event>::with_user_event().build();
    let proxy = event_loop.create_proxy();
    let window = WindowBuilder::new()
        .with_title("BoxLinkBox — URLワークスペース")
        .with_inner_size(LogicalSize::new(1280.0, 840.0))
        .with_min_inner_size(LogicalSize::new(860.0, 600.0))
        .build(&event_loop)
        .map_err(|_| "ウィンドウを作成できませんでした。")?;
    let html = include_str!("../ui/index.html")
        .replace("/*__CSS__*/", include_str!("../ui/style.css"))
        .replace("/*__JS__*/", include_str!("../ui/app.js"));
    let ipc = proxy.clone();
    let mut shell_context = WebContext::new(Some(root.join("interface-profile")));
    let ui = WebViewBuilder::new_with_web_context(&mut shell_context)
        .with_custom_protocol("boxlinkbox".into(), move |_, _| {
            let mut response =
                wry::http::Response::new(std::borrow::Cow::Owned(html.as_bytes().to_vec()));
            response.headers_mut().insert(
                wry::http::header::CONTENT_TYPE,
                wry::http::HeaderValue::from_static("text/html; charset=utf-8"),
            );
            response
        })
        .with_url("boxlinkbox://local/")
        .with_devtools(false)
        .with_hotkeys_zoom(false)
        .with_navigation_handler(|url| {
            url == "http://boxlinkbox.local/" || url == "boxlinkbox://local/"
        })
        .with_new_window_req_handler(|_, _| wry::NewWindowResponse::Deny)
        .with_ipc_handler(move |req| {
            if req.body().len() <= 64 * 1024 {
                let _ = ipc.send_event(Event::Command(req.into_body()));
            }
        })
        .build(&window)
        .map_err(|_| "WebView2を初期化できませんでした。")?;
    let mut browser = Browser::new(root.join("browser-profile"));
    let data_directory = root.to_string_lossy().into_owned();
    event_loop.run(move |event,_,control| {
        let _keep_instance_lock = &instance;
        *control=ControlFlow::Wait;
        let mut library_dirty = false;
        let result:Result<(),String>=(|| {
            match event {
                TaoEvent::WindowEvent { event:WindowEvent::CloseRequested,.. }=> { browser.tabs.clear(); diagnostics::record(&root,"shutdown"); *control=ControlFlow::Exit; },
                TaoEvent::WindowEvent { event:WindowEvent::Resized(_)|WindowEvent::ScaleFactorChanged{..},.. }=>browser.layout(&window)?,
                TaoEvent::UserEvent(Event::Command(raw))=> {
                    let command:Command=serde_json::from_str(&raw).map_err(|_|"操作を読み取れませんでした。")?;
                    let modifies_library = matches!(&command, Command::AddGroup{..}|Command::RenameGroup{..}|Command::DeleteGroup{..}|Command::SaveLink{..}|Command::DeleteLink{..}|Command::MoveLink{..}|Command::Theme{..}|Command::Import);
                    let mut candidate=if modifies_library { data.clone() } else { Library::default() }; let mut changed=false;
                    match command {
                        Command::Legal { document }=> {
                            let (title,text)=match document.as_str() {
                                "license"=>("BoxLinkBox MIT License",include_str!("../LICENSE")),
                                "licenses"=>("オープンソースライセンス",concat!(include_str!("../THIRD-PARTY-NOTICES.txt"),"\n\n",include_str!("../RUST-STDLIB-NOTICES.txt"))),
                                "privacy"=>("BoxLinkBox プライバシーポリシー",include_str!("../PRIVACY.md")),
                                "terms"=>("BoxLinkBox 利用について",include_str!("../TERMS.md")),
                                _=>return Err("説明文を開けませんでした。".into()),
                            };
                            send(&ui,json!({"kind":"document","title":title,"text":text}));
                        },
                        Command::Quit=> { browser.tabs.clear();diagnostics::record(&root,"shutdown");*control=ControlFlow::Exit; },
                        Command::Ready=> { library_dirty=true; if recovered { alert(&ui,"直前の保存データから復旧しました。内容を確認してください。"); } },
                        Command::AddGroup{name}=> { candidate.groups.push(Group {id:candidate.next_id(),name:name.trim().into(),links:vec![]});changed=true; },
                        Command::RenameGroup{id,name}=> { candidate.groups.iter_mut().find(|g|g.id==id).ok_or("グループがありません。")?.name=name.trim().into();changed=true; },
                        Command::DeleteGroup{id}=> { candidate.groups.retain(|g|g.id!=id);changed=true; },
                        Command::SaveLink{group,id,title,url,note}=> {
                            let url=normalize_url(&url)?; let next=candidate.next_id();
                            let g=candidate.groups.iter_mut().find(|g|g.id==group).ok_or("グループを選択してください。")?;
                            let link=Link {id:id.unwrap_or(next),title:title.trim().into(),url,note:note.trim().into()};
                            if let Some(id)=id { *g.links.iter_mut().find(|l|l.id==id).ok_or("URLがありません。")?=link; } else { g.links.push(link); }
                            changed=true;
                        },
                        Command::DeleteLink{group,id}=> { candidate.groups.iter_mut().find(|g|g.id==group).ok_or("グループがありません。")?.links.retain(|l|l.id!=id);changed=true; },
                        Command::MoveLink{group,id,direction}=> { let links=&mut candidate.groups.iter_mut().find(|g|g.id==group).ok_or("グループがありません。")?.links; if let Some(index)=links.iter().position(|l|l.id==id) { let target=if direction<0 { index.saturating_sub(1) } else { (index+1).min(links.len()-1) };links.swap(index,target);changed=true; } },
                        Command::Theme{theme}=> { candidate.theme=theme;changed=true; },
                        Command::Undo=> { if let Some(previous)=undo.last() { repo.save(previous)?;data=previous.clone();undo.pop();library_dirty=true;alert(&ui,"直前の変更を取り消しました。"); } },
                        Command::OpenGroup{id}=> {
                            let group=data.groups.iter().find(|g|g.id==id).ok_or("グループがありません。")?;
                            if group.links.is_empty() { browser.visible=false;browser.layout(&window)?; } else {
                                let missing:Vec<_>=group.links.iter().filter(|l|!browser.tabs.iter().any(|t|t.source_url==l.url)).cloned().collect();
                                if missing.len()+browser.tabs.len()>40 { return Err("一括オープンは合計40タブまでです。先にタブを閉じてください。".into()); }
                                for link in missing { browser.open(&window,&proxy,link.title,link.url)?; }
                                if let Some(tab)=browser.tabs.iter().find(|t|t.source_url==group.links[0].url) { browser.active=Some(tab.info.id); }
                                browser.visible=true;browser.layout(&window)?;
                            }
                        },
                        Command::OpenLink{group,id}=> { let link=data.groups.iter().find(|g|g.id==group).and_then(|g|g.links.iter().find(|l|l.id==id)).ok_or("URLがありません。")?;browser.open(&window,&proxy,link.title.clone(),link.url.clone())?; },
                        Command::SelectTab{id}=> { if browser.tabs.iter().any(|t|t.info.id==id) { browser.active=Some(id);browser.visible=true;browser.layout(&window)?; } },
                        Command::CloseTab{id}=> { browser.tabs.retain(|t|t.info.id!=id);if browser.active==Some(id) { browser.active=browser.tabs.last().map(|t|t.info.id); } browser.visible=browser.active.is_some();browser.layout(&window)?; },
                        Command::CloseAll=> { browser.tabs.clear();browser.active=None;browser.visible=false; },
                        Command::Manage=> { browser.visible=false;browser.layout(&window)?; },
                        Command::Browse=> { browser.visible=browser.active.is_some();browser.layout(&window)?; },
                        Command::Navigate{url}=> { let url=normalize_url(&url)?;if let Some(view)=browser.active_view() { view.load_url(&url).map_err(|_|"URLを開けませんでした。")?; } else { browser.open(&window,&proxy,"新しいタブ".into(),url)?; } },
                        Command::Back|Command::Forward|Command::Reload=> { if let Some(view)=browser.active_view() { match command {Command::Back=>view.go_back(),Command::Forward=>view.go_forward(),_=>view.reload()}.map_err(|_|"ページを操作できませんでした。")?; } },
                        Command::Export=> { browser.visible=false;browser.layout(&window)?; if let Some(path)=rfd::FileDialog::new().set_file_name("BoxLinkBox-backup.json").add_filter("JSON",&["json"]).save_file() { boxlinkbox::backup::export(&path,&data)?;alert(&ui,"バックアップを保存しました（ログイン情報は含みません）。"); } },
                        Command::Import=> { browser.visible=false;browser.layout(&window)?;if let Some(path)=rfd::FileDialog::new().add_filter("JSON",&["json"]).pick_file() { candidate=boxlinkbox::backup::import(&path)?;changed=true; } },
                    }
                    if changed { candidate.validate()?;repo.save(&candidate)?;undo.push(std::mem::replace(&mut data,candidate));library_dirty=true;send(&ui,json!({"kind":"saved"}));alert(&ui,"保存しました。"); }
                },
                TaoEvent::UserEvent(Event::Navigation(id,url))=> { if let Some(tab)=browser.tabs.iter_mut().find(|t|t.info.id==id) { tab.info.url=url; } },
                TaoEvent::UserEvent(Event::Title(id,title))=> { if let Some(tab)=browser.tabs.iter_mut().find(|t|t.info.id==id) { tab.info.title=title; } },
                TaoEvent::UserEvent(Event::Load(id,loading,url))=> { if let Some(tab)=browser.tabs.iter_mut().find(|t|t.info.id==id) { tab.info.loading=loading;tab.info.url=url; } },
                TaoEvent::UserEvent(Event::Popup(url))=> { browser.open(&window,&proxy,"新しいタブ".into(),normalize_url(&url)?)?; },
                _=>return Ok(()),
            }
            snapshot(&ui,if library_dirty { Some(&data) } else { None },&browser,!undo.is_empty(),&data_directory);Ok(())
        })();
        if let Err(error)=result { diagnostics::record(&root,"operation_failed");alert(&ui,&error);snapshot(&ui,if library_dirty { Some(&data) } else { None },&browser,!undo.is_empty(),&data_directory); }
    });
}
