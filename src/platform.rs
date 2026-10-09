use std::path::PathBuf;

pub fn data_directory() -> Result<PathBuf, String> {
    if let Some(directory) =
        std::env::var_os("BOXLINKBOX_DATA_DIR").filter(|value| !value.is_empty())
    {
        return Ok(PathBuf::from(directory));
    }
    let local = std::env::var_os("LOCALAPPDATA")
        .filter(|value| !value.is_empty())
        .ok_or("Windowsのデータ保存先を取得できませんでした。")?;
    // Keep this application separate from the Store edition.
    Ok(PathBuf::from(local).join("BoxLinkBox").join("data"))
}
