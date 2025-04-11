use reqwest::Url;
use serde::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[derive(Serialize, PartialEq, Deserialize, Clone)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi)
)]
pub struct NoteSearchV1 {
    name: String,
}
#[derive(Serialize, PartialEq, Deserialize, Clone, Copy)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi)
)]
pub struct NoteIDV1(pub u64);

#[derive(Serialize, PartialEq, Eq, Deserialize)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi)
)]
pub struct NoteV1 {
    rev: u64,
    sha256: String,
    note_data: NoteDataV1,
}
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
#[cfg_attr(
    target_arch = "wasm32",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi)
)]
pub enum NoteDataV1 {
    Text(String),
    Img(String),
}

#[derive(Serialize, PartialEq, Deserialize)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi)
)]
pub struct NoteUpdateV1 {
    note: NoteV1,
    note_id: NoteIDV1,
}
