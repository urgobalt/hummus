use reqwest::{Method, StatusCode};

use crate::error::Error;
use crate::request::RequestBackend;
use crate::{DefaultBackend, Note, NoteID, NoteUpdate};

pub async fn add_new_note<B: RequestBackend>(
    note: &Note,
    store: &str,
    cookie: &str,
) -> Result<StatusCode, Error> {
    B::do_json_status_request("api/note/new", Method::PUT, note, store, cookie).await
}
pub async fn update_note<B: RequestBackend>(
    note: &NoteUpdate,
    store: &str,
    cookie: &str,
) -> Result<(StatusCode, Option<Note>), Error> {
    B::do_json_json_request("api/note/update", Method::PATCH, note, store, cookie).await
}
pub async fn get_note_by_id<B: RequestBackend>(
    note: &NoteID,
    store: &str,
    cookie: &str,
) -> Result<(StatusCode, Option<Note>), Error> {
    B::do_json_json_request("api/note/update", Method::PATCH, note, store, cookie).await
}
