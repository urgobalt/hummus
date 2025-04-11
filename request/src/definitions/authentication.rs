use serde::{Deserialize, Serialize};

use self::v1::SessionV1;

use super::{AsAPIV1, Latest, Sealed};

pub mod v1;
pub type Session = v1::SessionV1;
///The Versioned NoteIDs see module level documentation
#[derive(Serialize, PartialEq, Deserialize, Clone)]
#[cfg_attr(target_arch = "wasm32", derive(tsify_next::Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub enum SessionVersion {
    V1(v1::SessionV1),
}

impl AsAPIV1<SessionV1> for  SessionVersion {
    fn to_v1(self) -> Result<v1::SessionV1, crate::Error> {
        Ok(match self {
            SessionVersion::V1(session_v1) => session_v1,
        })
    }
}
impl Sealed for SessionVersion {}
impl Latest<SessionV1,Session> for SessionVersion {
    fn get_latest(self) -> Session {
        match self {
            SessionVersion::V1(session) => session,
        }
    }
}
