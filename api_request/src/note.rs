use reqwest::{Method, StatusCode};
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::request::RequestBackend;
use crate::{Metadata, Session};

#[derive(Serialize, PartialEq, Deserialize, Clone)]
struct NoteSearch {
    name: String,
}
#[derive(Serialize, PartialEq, Deserialize, Clone, Copy)]
pub struct NoteID(pub u64);
#[derive(Serialize, PartialEq, Deserialize)]
pub struct Note {
    text: String,
}
#[derive(Serialize, PartialEq, Deserialize)]
pub struct NoteUpdate {
    note: Note,
    note_id: NoteID,
}
pub async fn get_note<B: RequestBackend>(
    note: NoteID,
    store: &str,
    cookie: &str,
) -> Result<(Metadata, Note), Error> {
    B::do_string_json_request::<true, Note>(
        &format!("api/note/{}", note.0),
        Method::PUT,
        "".to_string(),
        store,
        cookie,
    )
    .await
}
pub async fn add_new_note<B: RequestBackend>(
    note: &Note,
    session: &Session,
) -> Result<Metadata, Error> {
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
) -> Result<(Metadata, Option<Note>), Error> {
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
) -> Result<Metadata, Error> {
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
) -> Result<(Metadata, Vec<NoteID>), Error> {
    B::do_json_json_request(
        "api/notes/search",
        Method::GET,
        search,
        &session.store_url,
        &session.cookie,
    )
    .await
}
pub async fn get_all_notes<B: RequestBackend>(
    session: &Session,
) -> Result<(Metadata, Vec<NoteID>), Error> {
    B::do_string_json_request::<true, Vec<NoteID>>(
        "api/notes",
        Method::GET,
        "".to_string(),
        &session.store_url,
        &session.cookie,
    )
    .await
}
