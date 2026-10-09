use crate::Event;
use serde::Serialize;
use std::cell::Cell;
use tao::{
    dpi::{LogicalPosition, LogicalSize},
    event_loop::EventLoopProxy,
    window::Window,
};
#[cfg(target_os = "windows")]
use wry::{MemoryUsageLevel, WebViewExtWindows};
use wry::{NewWindowResponse, PageLoadEvent, Rect, WebContext, WebView, WebViewBuilder};

#[derive(Serialize)]
pub struct TabInfo {
    pub id: u64,
    pub title: String,
    pub url: String,
    pub loading: bool,
}
pub struct Tab {
    pub info: TabInfo,
    pub view: WebView,
    pub source_url: String,
    visible: Cell<Option<bool>>,
    geometry: Cell<Option<(u32, u32, u64)>>,
}
pub struct Browser {
    pub tabs: Vec<Tab>,
    pub active: Option<u64>,
    pub visible: bool,
    pub context: WebContext,
    next: u64,
}
pub fn bounds(window: &Window) -> Rect {
    let size = window.inner_size().to_logical::<f64>(window.scale_factor());
    Rect {
        position: LogicalPosition::new(246.0, 142.0).into(),
        size: LogicalSize::new(
            (size.width - 246.0).max(1.0),
            (size.height - 142.0).max(1.0),
        )
        .into(),
    }
}
impl Browser {
    pub fn new(path: std::path::PathBuf) -> Self {
        Self {
            tabs: vec![],
            active: None,
            visible: false,
            context: WebContext::new(Some(path)),
            next: 1,
        }
    }
    pub fn open(
        &mut self,
        window: &Window,
        proxy: &EventLoopProxy<Event>,
        title: String,
        url: String,
    ) -> Result<(), String> {
        if self.tabs.len() >= 40 {
            return Err("同時に開けるタブは40個までです。不要なタブを閉じてください。".into());
        }
        let id = self.next;
        self.next += 1;
        let nav = proxy.clone();
        let popup = proxy.clone();
        let load = proxy.clone();
        let heading = proxy.clone();
        let view = WebViewBuilder::new_with_web_context(&mut self.context)
            .with_url(&url)
            .with_bounds(bounds(window))
            .with_visible(false)
            .with_devtools(false)
            .with_navigation_handler(move |url| {
                if crate::domain_url(&url) {
                    let _ = nav.send_event(Event::Navigation(id, url));
                    true
                } else {
                    false
                }
            })
            .with_new_window_req_handler(move |url, _| {
                let _ = popup.send_event(Event::Popup(url));
                NewWindowResponse::Deny
            })
            .with_document_title_changed_handler(move |title| {
                let _ = heading.send_event(Event::Title(id, title));
            })
            .with_on_page_load_handler(move |event, url| {
                let _ = load.send_event(Event::Load(
                    id,
                    matches!(event, PageLoadEvent::Started),
                    url,
                ));
            })
            .build_as_child(window)
            .map_err(|_| "ページを開けませんでした。WebView2とメモリの空きを確認してください。")?;
        self.tabs.push(Tab {
            source_url: url.clone(),
            info: TabInfo {
                id,
                title,
                url,
                loading: true,
            },
            view,
            visible: Cell::new(None),
            geometry: Cell::new(None),
        });
        self.active = Some(id);
        self.visible = true;
        self.layout(window)?;
        Ok(())
    }
    pub fn layout(&self, window: &Window) -> Result<(), String> {
        let size = window.inner_size();
        let geometry = (size.width, size.height, window.scale_factor().to_bits());
        for tab in &self.tabs {
            if tab.geometry.get() != Some(geometry) {
                tab.view
                    .set_bounds(bounds(window))
                    .map_err(|_| "ブラウザのサイズを変更できませんでした。")?;
                tab.geometry.set(Some(geometry));
            }
            let visible = self.visible && self.active == Some(tab.info.id);
            if tab.visible.get() != Some(visible) {
                tab.view
                    .set_visible(visible)
                    .map_err(|_| "タブを切り替えられませんでした。")?;
                #[cfg(target_os = "windows")]
                {
                    // Older runtimes may not expose this optional memory hint.
                    let _ = tab.view.set_memory_usage_level(if visible {
                        MemoryUsageLevel::Normal
                    } else {
                        MemoryUsageLevel::Low
                    });
                }
                tab.visible.set(Some(visible));
            }
        }
        Ok(())
    }
    pub fn active_view(&self) -> Option<&WebView> {
        self.tabs
            .iter()
            .find(|t| Some(t.info.id) == self.active)
            .map(|t| &t.view)
    }
}
