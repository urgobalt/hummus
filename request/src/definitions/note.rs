mod v1;
pub type Note = v1::NoteV1;
pub type NoteID = v1::NoteIDV1;
pub type NoteSearch = v1::NoteSearchV1;
pub type NoteUpdate = v1::NoteUpdateV1;

use serde::{Deserialize, Serialize};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use super::{Latest, Sealed};
use v1::*;
#[derive(Serialize, PartialEq, Deserialize, Clone, Copy)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi)
)]
pub enum NoteIDVersion {
    V1(v1::NoteIDV1),
}

impl Sealed for NoteIDVersion {}
impl Latest<NoteID> for NoteIDVersion {
    fn get_latest(self) -> NoteID {
        match self {
            NoteIDVersion::V1(note_idv1) => note_idv1,
        }
    }
}
#[derive(Serialize, PartialEq, Deserialize)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi)
)]
pub enum NoteVersion {
    V1(v1::NoteV1),
}

impl Sealed for NoteVersion {}
impl Latest<Note> for NoteVersion {
    fn get_latest(self) -> Note {
        match self {
            NoteVersion::V1(note_v1) => note_v1,
        }
    }
}
#[derive(Serialize, PartialEq, Deserialize)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi)
)]
pub enum NoteUpdateVersion {
    V1(NoteUpdateV1),
}
impl Sealed for NoteUpdateVersion {}
impl Latest<NoteUpdate> for NoteUpdateVersion {
    fn get_latest(self) -> NoteUpdate {
        match self {
            NoteUpdateVersion::V1(note_idv1) => note_idv1,
        }
    }
}
#[derive(Serialize, PartialEq, Deserialize, Clone)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi)
)]
pub enum NoteSearchVersion {
    V1(v1::NoteSearchV1),
}
impl Sealed for NoteSearchVersion {}
impl Latest<NoteSearch> for NoteSearchVersion {
    fn get_latest(self) -> NoteSearch {
        match self {
            NoteSearchVersion::V1(note_search_v1) => note_search_v1,
        }
    }
}
