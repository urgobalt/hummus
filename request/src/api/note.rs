use super::v1;
use crate::definitions::AsAPIV1;
use crate::definitions::authentication::SessionVersion;
use crate::definitions::note::{NoteIDVersion, NoteSearchVersion, NoteUpdateVersion, NoteVersion};
use crate::{RequestBackend, ResponseResult};
///
#[inline(always)]
pub async fn get_note<B: RequestBackend>(
    note: NoteIDVersion,
    session: &SessionVersion,
) -> ResponseResult<NoteVersion> {
    match note {
        NoteIDVersion::V1(note_idv1) => v1::get_note::<B>(note_idv1, &session.clone().to_v1()?)
            .await
            .map(|v| v.map_body(NoteVersion::V1)),
    }
}
#[inline(always)]
pub async fn add_new_note<B: RequestBackend>(
    note: &NoteVersion,
    session: &SessionVersion,
) -> ResponseResult<()> {
    match note {
        NoteVersion::V1(note_idv1) => {
            v1::add_new_note::<B>(note_idv1, &session.clone().to_v1()?).await
        }
    }
}
#[inline(always)]
pub async fn update_note<B: RequestBackend>(
    note: &NoteUpdateVersion,
    session: &SessionVersion,
) -> ResponseResult<Option<NoteVersion>> {
    match note {
        NoteUpdateVersion::V1(note_idv1) => {
            v1::update_note::<B>(note_idv1, &session.clone().to_v1()?)
                .await
                .map(|v| v.map_body(|v| v.map(NoteVersion::V1)))
        }
    }
}
#[inline(always)]
pub async fn delete_note<B: RequestBackend>(
    note: &NoteIDVersion,
    session: &SessionVersion,
) -> ResponseResult<()> {
    match note {
        NoteIDVersion::V1(note_idv1) => {
            v1::delete_note::<B>(note_idv1, &session.clone().to_v1()?).await
        }
    }
}
#[inline(always)]
pub async fn search_notes<B: RequestBackend>(
    search: &NoteSearchVersion,
    session: &SessionVersion,
) -> ResponseResult<Vec<NoteIDVersion>> {
    match search {
        NoteSearchVersion::V1(note_idv1) => {
            v1::search_notes::<B>(note_idv1, &session.clone().to_v1()?)
                .await
                .map(|v| v.map_body(|v| v.into_iter().map(NoteIDVersion::V1).collect()))
        }
    }
}
#[inline(always)]
pub async fn get_all_notes<B: RequestBackend>(
    session: &SessionVersion,
) -> ResponseResult<Vec<NoteIDVersion>> {
    match session {
        SessionVersion::V1(note_idv1) => {
            v1::get_all_notes::<B>(note_idv1)
                .await
                .map(|v| v.map_body(|v| v.into_iter().map(NoteIDVersion::V1).collect()))
        }
    }
}
