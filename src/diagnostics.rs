use std::{
    fs::OpenOptions,
    io::Write,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

// Only static event codes are recorded. URLs, UI input and errors are excluded.
pub fn record(root: &Path, code: &'static str) {
    let path = root.join("app.log");
    let truncate = std::fs::metadata(&path).is_ok_and(|m| m.len() > 64 * 1024);
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .write(true)
        .append(!truncate)
        .truncate(truncate)
        .open(path)
    {
        let seconds = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let _ = writeln!(file, "{seconds} {code}");
    }
}
