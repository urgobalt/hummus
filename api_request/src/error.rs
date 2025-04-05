pub enum Error {
    Reqwest(reqwest::Error),
    Axum(String),
    #[cfg(all(feature = "tauri", target_arch = "wasm32"))]
    WasmSerde(serde_wasm_bindgen::Error),
    #[cfg(all(feature = "tauri", target_arch = "wasm32"))]
    WasmTauri(tauri_wasm::Error),
    Serde(serde_json::Error),
}
impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::Reqwest(value)
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
        Self::WasmSerde(value)
    }
}
#[cfg(all(feature = "tauri", target_arch = "wasm32"))]
impl From<tauri_wasm::Error> for Error {
    fn from(value: tauri_wasm::Error) -> Self {
        Self::WasmTauri(value)
    }
}
impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde(value)
    }
}
