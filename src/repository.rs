use crate::domain::Library;
use rusqlite::{Connection, params};
use std::path::Path;

pub struct Repository {
    connection: Connection,
}
impl Repository {
    pub fn open(path: &Path) -> Result<Self, String> {
        let connection = Connection::open(path)
            .map_err(|_| "保存データを開けません。保存先の権限を確認してください。")?;
        connection.execute_batch("PRAGMA journal_mode=WAL; PRAGMA synchronous=FULL; CREATE TABLE IF NOT EXISTS library (id INTEGER PRIMARY KEY CHECK(id=1), data TEXT NOT NULL); CREATE TABLE IF NOT EXISTS recovery (id INTEGER PRIMARY KEY CHECK(id=1), data TEXT NOT NULL);").map_err(|_| "保存領域を準備できませんでした。")?;
        Ok(Self { connection })
    }
    pub fn load(&self) -> Result<Library, String> {
        let result = self
            .connection
            .query_row("SELECT data FROM library WHERE id=1", [], |r| {
                r.get::<_, String>(0)
            });
        match result {
            Ok(json) => {
                let value: Library = serde_json::from_str(&json)
                    .map_err(|_| "保存データが壊れています。バックアップから復元してください。")?;
                value.validate()?;
                Ok(value)
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(Library::default()),
            Err(_) => Err("保存データを読み込めませんでした。".into()),
        }
    }
    pub fn save(&mut self, data: &Library) -> Result<(), String> {
        data.validate()?;
        let json = serde_json::to_string(data).map_err(|_| "保存データを作成できませんでした。")?;
        let transaction = self
            .connection
            .transaction()
            .map_err(|_| "保存を開始できませんでした。")?;
        transaction
            .execute(
                "INSERT OR REPLACE INTO recovery SELECT id,data FROM library WHERE id=1",
                [],
            )
            .map_err(|_| "復旧データを保存できませんでした。")?;
        transaction
            .execute(
                "INSERT OR REPLACE INTO library(id,data) VALUES(1,?1)",
                params![json],
            )
            .map_err(|_| "保存できませんでした。ディスクの空き容量を確認してください。")?;
        transaction
            .commit()
            .map_err(|_| "保存を確定できませんでした。".to_owned())
    }
    pub fn recover(&self) -> Result<Library, String> {
        let json: String = self
            .connection
            .query_row("SELECT data FROM recovery WHERE id=1", [], |r| r.get(0))
            .map_err(|_| "復旧用データがありません。")?;
        let data: Library =
            serde_json::from_str(&json).map_err(|_| "復旧用データも読み込めませんでした。")?;
        data.validate()?;
        Ok(data)
    }
}
