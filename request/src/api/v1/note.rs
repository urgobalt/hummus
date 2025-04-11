use crate::Session;
use crate::definitions::{
    Latest, NoteIDVersion, NoteSearchVersion, NoteUpdateVersion, NoteVersion,
};
use crate::{RequestBackend, ResponseResult};
use reqwest::Method;
pub async fn get_note<B: RequestBackend>(
    note: NoteIDVersion,
    store: &str,
    cookie: &str,
) -> ResponseResult<NoteVersion> {
    B::do_string_json_request::<true, NoteVersion>(
        &format!("api/note/{}", note.get_latest().0),
        Method::PUT,
        "".to_string(),
        store,
        cookie,
    )
    .await
}
pub async fn add_new_note<B: RequestBackend>(
    note: &NoteVersion,
    session: &Session,
) -> ResponseResult<()> {
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
    note: &NoteUpdateVersion,
    session: &Session,
) -> ResponseResult<Option<NoteVersion>> {
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
    note: &NoteIDVersion,
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
    search: &NoteSearchVersion,
    session: &Session,
) -> ResponseResult<Vec<NoteIDVersion>> {
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
) -> ResponseResult<Vec<NoteIDVersion>> {
    B::do_string_json_request::<true, Vec<NoteIDVersion>>(
        "api/notes",
        Method::GET,
        "".to_string(),
        &session.store_url,
        &session.cookie,
    )
    .await
}
