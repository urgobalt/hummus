#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg_attr(
    target_arch = "wasm32",
    derive(tsify_next::Tsify, serde::Serialize),
    tsify(into_wasm_abi),
    tsify(namespace)
)]
pub enum Error {
    Reqwest(String),
    Axum(String),
    WasmSerde(String),
    WasmTauri(String),
    Serde(String),
}
impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        return Self::Reqwest(value.to_string());
    }
}
#[cfg(all(feature = "tauri", not(target_arch = "wasm32")))]
impl From<axum::http::Error> for Error {
    fn from(value: axum::http::Error) -> Self {
        Self::Axum(value.to_string())
    }
}
#[cfg(all(feature = "tauri", not(target_arch = "wasm32")))]
impl From<axum::Error> for Error {
    fn from(value: axum::Error) -> Self {
        Self::Axum(value.to_string())
    }
}
#[cfg(all(feature = "tauri", target_arch = "wasm32"))]
impl From<serde_wasm_bindgen::Error> for Error {
    fn from(value: serde_wasm_bindgen::Error) -> Self {
        Self::WasmSerde(value.to_string())
    }
}
#[cfg(all(feature = "tauri", target_arch = "wasm32"))]
impl From<tauri_wasm::Error> for Error {
    fn from(value: tauri_wasm::Error) -> Self {
        Self::WasmTauri(value.to_string())
    }
}
impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        return Self::Serde(value.to_string());
    }
}
