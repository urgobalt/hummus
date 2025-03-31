#![allow(async_fn_in_trait)]
pub mod error;
pub mod note;
mod request;
pub use request::*;
use serde::{Deserialize, Serialize};
#[derive(Serialize, PartialEq, Deserialize)]
pub struct Note {
    text: String,
}
#[derive(Serialize, PartialEq, Deserialize)]
pub struct NoteUpdate {
    note: Note,
    note_id: NoteID,
}
#[derive(Serialize, PartialEq, Deserialize)]
pub struct StoreID(pub String);
pub const DEFAULT_HOST: &'static str = "localhost:3000";
#[derive(Serialize, PartialEq, Deserialize)]
pub struct NoteID(pub u64);
#[derive(Serialize, PartialEq, Deserialize)]
pub struct UserID(pub u64);

#[derive(Serialize, PartialEq, Deserialize)]
pub struct StoreSession(pub u64);
#[derive(Serialize, PartialEq, Deserialize)]
pub struct IDSSession(pub u64);

#[derive(Serialize, PartialEq, Deserialize, Clone)]
pub struct ServerState {}

pub fn ensure_valid_base_url(url: &str) -> &str {
    url.trim_end_matches('/')
}
