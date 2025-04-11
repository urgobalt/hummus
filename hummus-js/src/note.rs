use hummus_macros::specialize_function_for_bindgen;
use hummus_request::definitions::authentication::SessionVersion;
use hummus_request::definitions::note::{
    NoteIDVersion, NoteSearchVersion, NoteUpdateVersion, NoteVersion,
};
use hummus_request::{DefaultBackend, ResponseResult};
specialize_function_for_bindgen!(
    hummus_request::api::note,
    DefaultBackend;

    {
        get_note,
        (
            note: NoteIDVersion,
            session: &SessionVersion
        ),
        -> ResponseResult<NoteVersion>
        },

    {
        add_new_note,
        (
            note: &NoteVersion,
            session: &SessionVersion
        ),
        -> ResponseResult<()>
    },     {
        update_note,
        (
            note: &NoteUpdateVersion,
            session: &SessionVersion
        ),
        -> ResponseResult<Option<NoteVersion>>
    },

    {
        delete_note,
        (
            note: &NoteIDVersion,
            session: &SessionVersion
        ),
        -> ResponseResult<()>
    },

    {
        search_notes,
        (
            search: &NoteSearchVersion,
            session: &SessionVersion
        ),
        -> ResponseResult<Vec<NoteIDVersion>>
    },
    {
        get_all_notes,
        (
            session: &SessionVersion
        ),
        -> ResponseResult<Vec<NoteIDVersion>>
    }
);
