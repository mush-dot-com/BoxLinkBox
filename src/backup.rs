use crate::domain::{Library, normalize_url};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

pub const MAX_BACKUP_BYTES: u64 = 16 * 1024 * 1024;
static NEXT_FILE: AtomicU64 = AtomicU64::new(1);

pub fn import(path: &Path) -> Result<Library, String> {
    let file = File::open(path).map_err(|_| "ファイルを読めませんでした。")?;
    let mut bytes = Vec::new();
    file.take(MAX_BACKUP_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| "ファイルを読めませんでした。")?;
    if bytes.len() as u64 > MAX_BACKUP_BYTES {
        return Err("バックアップは16MB以下にしてください。".into());
    }
    let mut library: Library =
        serde_json::from_slice(&bytes).map_err(|_| "BoxLinkBoxのバックアップではありません。")?;
    for group in &mut library.groups {
        for link in &mut group.links {
            link.url = normalize_url(&link.url)?;
        }
    }
    library.validate()?;
    Ok(library)
}

pub fn export(path: &Path, library: &Library) -> Result<(), String> {
    library.validate()?;
    let bytes =
        serde_json::to_vec_pretty(library).map_err(|_| "バックアップを作成できませんでした。")?;
    if bytes.len() as u64 > MAX_BACKUP_BYTES {
        return Err("バックアップが16MBを超えます。メモや登録件数を減らしてください。".into());
    }
    let parent = path.parent().ok_or("保存先を確認してください。")?;
    let mut temporary = None;
    for _ in 0..32 {
        let sequence = NEXT_FILE.fetch_add(1, Ordering::Relaxed);
        let candidate = parent.join(format!(".boxlinkbox-{}-{sequence}.tmp", std::process::id()));
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&candidate)
        {
            Ok(file) => {
                temporary = Some((candidate, file));
                break;
            }
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(_) => return Err("バックアップの一時ファイルを作成できませんでした。".into()),
        }
    }
    let (temporary_path, mut file) = temporary.ok_or("バックアップを作成できませんでした。")?;
    let written = file.write_all(&bytes).and_then(|_| file.sync_all());
    drop(file);
    let result = written.and_then(|_| std::fs::rename(&temporary_path, path));
    if result.is_err() {
        let _ = std::fs::remove_file(&temporary_path);
        return Err("バックアップを保存できませんでした。既存のファイルは保持しました。".into());
    }
    Ok(())
}
