use axum::Router;
use axum::routing::get;

pub fn router() -> Router {
    Router::new().route("/hello", get(async || "hello"))
}
pub struct Note {
    text: String,
}
pub struct NoteID(pub u64);
pub struct OutsideID(pub u64);
pub struct InsideID(pub u64);
pub struct InsideSession(pub u64);
pub struct OutsideSession(pub u64);
// combined outside_session and inside_session
pub enum Session {
    OutsideSession(OutsideSession),
    InsideSession(InsideSession),
}
// Operations on an account regardless of what it is
pub enum Account {
    Local,
    OutisdeAccount(OutisdeAccount),
    InsideAccount(InsideAccount),
}

impl OutsideID {
    pub fn new() -> OutsideID {
        todo!("create a new unique outside id");
    }
}
impl InsideID {
    pub fn new() -> InsideID {
        todo!("create a new unique inside id");
    }
}
pub struct OutisdeAccount {
    pub id: OutsideID,
    pub name: String,
    password: String, // replace this please
}
pub struct InsideAccount {
    pub id: InsideID,
    pub name: String,
    password: String, // replace this please
}
// Describes **Interactions** with the inside server that are required.
mod inside_server {
    use super::*;

    pub fn create_inside_server_acount(name: String, password: String) -> InsideAccount {
        InsideAccount {
            id: InsideID::new(),
            name,
            password,
        }
    }

    pub fn move_notes_to_inside_server(to_session: InsideSession, note: Vec<Note>) {
        todo!("Transfer notes to inside")
    }
    pub fn save_to_inside_server(id: InsideID, note: Note) {
        todo!("Save to DB")
    }
    pub fn get_note(note_id: NoteID) {}
}
// Describes **Interactions** with any outside server that are required.
mod outside_server {
    use super::*;

    pub fn move_acount_from_outside_to_inside(to_session: InsideSession, from_id: OutsideID) {
        todo!("Transfer notes using inside_server::move_notes_to_id")
    }
    pub fn move_notes_to_outside_server(to_session: OutsideSession, note: Vec<Note>) {
        todo!("Transfer notes to inside")
    }
    pub fn create_outside_server_acount(name: String, password: String) -> OutisdeAccount {
        OutisdeAccount {
            id: OutsideID::new(),
            name,
            password,
        }
    }
    pub fn save_to_outside_server(id: OutsideID, note: Note) {
        todo!("Save to DB")
    }
    pub fn move_acoutn_from_outside_to_outside(othersession:OutsideSession, current:OutisdeAccount){
    }
}
mod local {
    use crate::*;

    //pub fn create_local_acount(){}
    // creation not needed
    pub fn save_to_local_server(note: Note) {
        todo!("Save to DB")
    }
    pub fn sync_note(
        session: Session,
        last_sync_note: Note,
        current_local_note: Note,
        current_server_note: Note,
    ) {
        todo!("Here we have kept the last synced state and then diff these, see what can be applied and then later we push an remove append with smth like merged so that it only need to diff from that, if we get in a thing that is merged, then it should be saved to the sate of the merge and then back saved so that other clients can synk to it or smth ")
    }
}
