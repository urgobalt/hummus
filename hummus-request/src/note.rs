use crate::error::Error;
use crate::request::RequestBackend;
use crate::{Metadata, ResponseResult, Session};
use reqwest::Method;
use serde::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use tsify_next::Tsify;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[derive(Serialize, PartialEq, Deserialize, Clone)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi)
)]
pub struct NoteSearch {
    name: String,
}
#[derive(Serialize, PartialEq, Deserialize, Clone, Copy)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi)
)]
pub struct NoteID(pub u64);

#[derive(Serialize, PartialEq, Deserialize)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi)
)]
pub struct Note {
    text: String,
}

#[derive(Serialize, PartialEq, Deserialize)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi)
)]
pub struct NoteUpdate {
    note: Note,
    note_id: NoteID,
}
pub async fn get_note<B: RequestBackend>(
    note: NoteID,
    store: &str,
    cookie: &str,
) -> ResponseResult<Note> {
    B::do_string_json_request::<true, Note>(
        &format!("api/note/{}", note.0),
        Method::PUT,
        "".to_string(),
        store,
        cookie,
    )
    .await
}
pub async fn add_new_note<B: RequestBackend>(note: &Note, session: &Session) -> ResponseResult<()> {
    B::do_json_status_request(
        "api/note",
        Method::PUT,
        note,
        &session.store_url,
        &session.cookie,
    )
    .await
}
pub async fn update_note<B: RequestBackend>(
    note: &NoteUpdate,
    session: &Session,
) -> ResponseResult<Option<Note>> {
    B::do_json_json_request(
        "api/note",
        Method::PATCH,
        note,
        &session.store_url,
        &session.cookie,
    )
    .await
}
pub async fn delete_note<B: RequestBackend>(
    note: &NoteID,
    session: &Session,
) -> ResponseResult<()> {
    B::do_json_status_request(
        "api/note",
        Method::DELETE,
        note,
        &session.store_url,
        &session.cookie,
    )
    .await
}
pub async fn search_notes<B: RequestBackend>(
    search: &NoteSearch,
    session: &Session,
) -> ResponseResult<Vec<NoteID>> {
    B::do_json_json_request(
        "api/notes/search",
        Method::GET,
        search,
        &session.store_url,
        &session.cookie,
    )
    .await
}
pub async fn get_all_notes<B: RequestBackend>(session: &Session) -> ResponseResult<Vec<NoteID>> {
    B::do_string_json_request::<true, Vec<NoteID>>(
        "api/notes",
        Method::GET,
        "".to_string(),
        &session.store_url,
        &session.cookie,
    )
    .await
}
