use hummus_macros::specialize_function_for_bindgen;
use hummus_request::{DefaultBackend, Error, ResponseResult};
use hummus_request::{
    Response, Session,
    note::{Note, NoteID, NoteSearch, NoteUpdate},
};
use tsify_next::declare;

#[declare]
pub type NoteResponse = Response<Note>;
#[declare]
pub type MaybeNoteResponse = Response<Option<Note>>;
#[declare]
pub type NoteIDResponse = Response<NoteID>;
#[declare]
pub type StatusResponse = Response<()>;
#[declare]
pub type NoteIDSResponse = Response<Vec<NoteID>>;
specialize_function_for_bindgen!(
    hummus_request::note,
    DefaultBackend;

    {
        get_note,
        (
            note: NoteID,
            store: &str,
            cookie: &str,
        ),
        -> Result<NoteResponse,Error>
        },

    {
        add_new_note,
        (
            note: &Note,
            session: &Session,
        ),
        -> Result<StatusResponse,Error>
    },     {
        update_note,
        (
            note: &NoteUpdate,
            session: &Session,
        ),
        -> Result<MaybeNoteResponse,Error>
    },

    {
        delete_note,
        (
            note: &NoteID,
            session: &Session,
        ),
        -> Result<StatusResponse, Error>
    },

    {
        search_notes,
        (
            search: &NoteSearch,
            session: &Session,
        ),
        -> Result<NoteIDSResponse,Error>
    },
    {
        get_all_notes,
        (
            session: &Session,
        ),
        -> Result<NoteIDSResponse,Error>
    }
);
