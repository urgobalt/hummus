pub struct Note {
    text: String,
}
pub struct NoteID(pub u64);
pub struct UserID(pub u64);

pub struct StoreSession(pub u64);
pub struct IDSSession(pub u64);
pub struct ServerState{}

/// combined outside_session and inside_session
pub enum Session {
    IDSSession(IDSSession),
    StoreSession(StoreSession),
}

mod shared{

}
