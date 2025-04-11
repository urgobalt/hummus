use serde::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::definitions::APIV1;

use super::Sealed;
#[derive(Serialize, PartialEq, Deserialize, Clone)]
#[cfg_attr(target_arch = "wasm32", derive(tsify_next::Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct NoteSearchV1 {
    name: String,
}
impl Sealed for NoteSearchV1 {}
impl APIV1 for NoteSearchV1 {}
#[derive(Serialize, PartialEq, Deserialize, Clone, Copy)]
#[cfg_attr(target_arch = "wasm32", derive(tsify_next::Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct NoteIDV1(pub u64);

impl Sealed for NoteIDV1 {}
impl APIV1 for NoteIDV1 {}
#[derive(Serialize, PartialEq, Eq, Deserialize)]
#[cfg_attr(target_arch = "wasm32", derive(tsify_next::Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct NoteV1 {
    rev: u64,
    sha256: String,
    note_data: NoteDataV1,
}
impl Sealed for NoteV1 {}
impl APIV1 for NoteV1 {}
impl PartialOrd for NoteV1 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.rev.partial_cmp(&other.rev)
    }
}
impl Ord for NoteV1 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rev.cmp(&other.rev)
    }
}
#[derive(Serialize, PartialEq, Eq, Deserialize)]
#[cfg_attr(target_arch = "wasm32", derive(tsify_next::Tsify), tsify(into_wasm_abi, from_wasm_abi))]
/// The data contained inside of type
pub enum NoteDataV1 {
    /// This is normal text
    Text(NoteText),
    /// This must be a valid b64 encoded image
    Img(NoteImage),
    /// This must be a valid url
    Interactive(NoteShared),
}

/// This must be a valid b64 encoded image
#[derive(Serialize, PartialEq, Eq, Deserialize)]
#[cfg_attr(target_arch = "wasm32", derive(tsify_next::Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct NoteImage(pub String);

/// This must be a valid url
#[derive(Serialize, PartialEq, Eq, Deserialize)]
#[cfg_attr(target_arch = "wasm32", derive(tsify_next::Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct NoteShared(pub String);

/// This is normal text that is transfered
#[derive(Serialize, PartialEq, Eq, Deserialize)]
#[cfg_attr(target_arch = "wasm32", derive(tsify_next::Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct NoteText(pub String);

#[derive(Serialize, PartialEq, Deserialize)]
#[cfg_attr(target_arch = "wasm32", derive(tsify_next::Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct NoteUpdateV1 {
    note: NoteV1,
    note_id: NoteIDV1,
}
impl Sealed for NoteUpdateV1 {}
impl APIV1 for NoteUpdateV1 {}
