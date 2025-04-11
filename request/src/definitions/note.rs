pub mod v1;
pub type Note = v1::NoteV1;
pub type NoteID = v1::NoteIDV1;
pub type NoteSearch = v1::NoteSearchV1;
pub type NoteUpdate = v1::NoteUpdateV1;

use serde::{Deserialize, Serialize};
use v1::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use super::{AsAPIV1, Latest, Sealed};
///The Versioned NoteIDs see module level documentation
#[derive(Serialize, PartialEq, Deserialize, Clone, Copy)]
#[cfg_attr(target_arch = "wasm32", derive(tsify_next::Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub enum NoteIDVersion {
    V1(v1::NoteIDV1),
}
impl AsAPIV1<v1::NoteIDV1> for NoteIDVersion {
    fn to_v1(self) -> Result<v1::NoteIDV1, crate::Error> {
        Ok(match self {
            NoteIDVersion::V1(session_v1) => session_v1,
        })
    }
}
impl NoteIDVersion {
    pub fn get_id_path(&self) -> String {
        match self {
            NoteIDVersion::V1(note_idv1) => format!("{}", note_idv1.0),
        }
    }
}

impl Sealed for NoteIDVersion {}
impl Latest<NoteIDV1, NoteID> for NoteIDVersion {
    fn get_latest(self) -> NoteID {
        match self {
            NoteIDVersion::V1(note_idv1) => note_idv1,
        }
    }
}

///The Versioned NoteIDs see module level documentation
#[derive(Serialize, PartialEq, Deserialize)]
#[cfg_attr(target_arch = "wasm32", derive(tsify_next::Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub enum NoteVersion {
    V1(v1::NoteV1),
}

impl AsAPIV1<v1::NoteV1> for NoteVersion {
    fn to_v1(self) -> Result<v1::NoteV1, crate::Error> {
        Ok(match self {
            NoteVersion::V1(session_v1) => session_v1,
        })
    }
}
impl Sealed for NoteVersion {}
impl Latest<NoteV1, Note> for NoteVersion {
    fn get_latest(self) -> Note {
        match self {
            NoteVersion::V1(note_v1) => note_v1,
        }
    }
}

///The Versioned NoteIDs see module level documentation
#[derive(Serialize, PartialEq, Deserialize)]
#[cfg_attr(target_arch = "wasm32", derive(tsify_next::Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub enum NoteUpdateVersion {
    V1(NoteUpdateV1),
}
impl Sealed for NoteUpdateVersion {}
impl Latest<NoteUpdateV1, NoteUpdate> for NoteUpdateVersion {
    fn get_latest(self) -> NoteUpdate {
        match self {
            NoteUpdateVersion::V1(note_idv1) => note_idv1,
        }
    }
}
impl AsAPIV1<v1::NoteUpdateV1> for NoteUpdateVersion {
    fn to_v1(self) -> Result<v1::NoteUpdateV1, crate::Error> {
        Ok(match self {
            NoteUpdateVersion::V1(session_v1) => session_v1,
        })
    }
}

///The Versioned NoteIDs see module level documentation
#[derive(Serialize, PartialEq, Deserialize, Clone)]
#[cfg_attr(target_arch = "wasm32", derive(tsify_next::Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub enum NoteSearchVersion {
    V1(v1::NoteSearchV1),
}
impl Sealed for NoteSearchVersion {}
impl Latest<NoteSearchV1, NoteSearch> for NoteSearchVersion {
    fn get_latest(self) -> NoteSearch {
        match self {
            NoteSearchVersion::V1(note_search_v1) => note_search_v1,
        }
    }
}
impl AsAPIV1<v1::NoteSearchV1> for NoteSearchVersion {
    fn to_v1(self) -> Result<v1::NoteSearchV1, crate::Error> {
        Ok(match self {
            NoteSearchVersion::V1(session_v1) => session_v1,
        })
    }
}
