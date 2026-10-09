use crate::domain::Library;
use serde::Serialize;

#[derive(Serialize)]
pub struct State<'a, T: Serialize> {
    pub kind: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub library: Option<&'a Library>,
    pub tabs: T,
    pub active: Option<u64>,
    pub browsing: bool,
    pub undo: bool,
    pub data_directory: &'a str,
}
