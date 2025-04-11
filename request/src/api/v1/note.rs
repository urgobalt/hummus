use reqwest::Method;

use crate::definitions::authentication::v1::SessionV1;
use crate::definitions::note::v1::{NoteIDV1, NoteSearchV1, NoteUpdateV1, NoteV1};
use crate::{RequestBackend, ResponseResult};
///
#[inline(always)]
pub async fn get_note<B: RequestBackend>(
    note: NoteIDV1,
    session: &SessionV1,
) -> ResponseResult<NoteV1> {
    B::do_string_json_request::<true, NoteV1>(
        &format!("api/v1/note/{}", note.0),
        Method::PUT,
        "".to_string(),
        session.get_store_url(),
        session.get_cookie(),
    )
    .await
}
#[inline(always)]
pub async fn add_new_note<B: RequestBackend>(
    note: &NoteV1,
    session: &SessionV1,
) -> ResponseResult<()> {
    B::do_json_status_request(
        "api/v1/note",
        Method::PUT,
        note,
        session.get_store_url(),
        session.get_cookie(),
    )
    .await
}
#[inline(always)]
pub async fn update_note<B: RequestBackend>(
    note: &NoteUpdateV1,
    session: &SessionV1,
) -> ResponseResult<Option<NoteV1>> {
    B::do_json_json_request(
        "api/v1/note",
        Method::PATCH,
        note,
        session.get_store_url(),
        session.get_cookie(),
    )
    .await
}
#[inline(always)]
pub async fn delete_note<B: RequestBackend>(
    note: &NoteIDV1,
    session: &SessionV1,
) -> ResponseResult<()> {
    B::do_json_status_request(
        "api/v1/note",
        Method::DELETE,
        note,
        session.get_store_url(),
        session.get_cookie(),
    )
    .await
}
#[inline(always)]
pub async fn search_notes<B: RequestBackend>(
    search: &NoteSearchV1,
    session: &SessionV1,
) -> ResponseResult<Vec<NoteIDV1>> {
    B::do_json_json_request(
        "api/notes/search",
        Method::GET,
        search,
        session.get_store_url(),
        session.get_cookie(),
    )
    .await
}
#[inline(always)]
pub async fn get_all_notes<B: RequestBackend>(
    session: &SessionV1,
) -> ResponseResult<Vec<NoteIDV1>> {
    B::do_string_json_request::<true, Vec<NoteIDV1>>(
        "api/notes",
        Method::GET,
        "".to_string(),
        session.get_store_url(),
        session.get_cookie(),
    )
    .await
}
