use hummus_macros::specialize_function_for_bindgen;
use hummus_request::{DefaultBackend, ResponseResult};
use hummus_request::{
    Session,
    definitions::{NoteIDVersion, NoteSearchVersion, NoteUpdateVersion, NoteVersion},
};
specialize_function_for_bindgen!(
    hummus_request::api::note,
    DefaultBackend;

    {
        get_note,
        (
            note: NoteIDVersion,
            store: &str,
            cookie: &str,
        ),
        -> ResponseResult<NoteVersion>
        },

    {
        add_new_note,
        (
            note: &NoteVersion,
            session: &Session,
        ),
        -> ResponseResult<()>
    },     {
        update_note,
        (
            note: &NoteUpdateVersion,
            session: &Session,
        ),
        -> ResponseResult<Option<NoteVersion>>
    },

    {
        delete_note,
        (
            note: &NoteIDVersion,
            session: &Session,
        ),
        -> ResponseResult<()>
    },

    {
        search_notes,
        (
            search: &NoteSearchVersion,
            session: &Session,
        ),
        -> ResponseResult<Vec<NoteIDVersion>>
    },
    {
        get_all_notes,
        (
            session: &Session,
        ),
        -> ResponseResult<Vec<NoteIDVersion>>
    }
);
