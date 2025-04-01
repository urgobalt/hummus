use crate::error::Error;
use reqwest::header::HeaderMap;
use reqwest::{Method, StatusCode};
use serde::{Serialize, de::DeserializeOwned};
pub struct Metadata {
    headers: HeaderMap,
    status: StatusCode,
}
impl Metadata {
    fn new(headers: HeaderMap, status: StatusCode) -> Self {
        Self { headers, status }
    }
}
impl From<(&HeaderMap, StatusCode)> for Metadata {
    fn from((headers, status): (&HeaderMap, StatusCode)) -> Self {
        Self {
            headers: headers.clone(),
            status,
        }
    }
}
impl From<(StatusCode, &HeaderMap)> for Metadata {
    fn from((status, headers): (StatusCode, &HeaderMap)) -> Self {
        Self {
            status,
            headers: headers.clone(),
        }
    }
}
pub const JSON_CONTENT_TYPE: &str = "application/json";
mod native;
mod wasm;
pub use native::Native;
#[cfg(all(feature = "tauri", target_arch = "wasm32"))]
pub type DefaultBackend = wasm::WasmTauri;

#[cfg(all(feature = "tauri", not(target_arch = "wasm32")))]
pub type DefaultBackend = wasm::Axum;

#[cfg(not(feature = "tauri"))]
pub type DefaultBackend = Native;

pub trait RequestBackend {
    #[cfg(all(feature = "tauri", target_arch = "wasm32"))]
    const DEFAULT: WasmTauri = WasmTauri;
    #[cfg(all(feature = "tauri", not(target_arch = "wasm32")))]
    const DEFAULT: wasm::Axum = wasm::Axum;
    #[cfg(not(feature = "tauri"))]
    const DEFAULT: Native = Native;
    async fn do_json_json_request<T: Serialize, R: DeserializeOwned>(
        url: &str,
        method: Method,
        body: &T,
        base_url: &str,
        cookie: &str,
    ) -> Result<(Metadata, R), Error>;
    async fn do_string_json_request<const JSON: bool, R: DeserializeOwned>(
        url: &str,
        method: Method,
        body: String,
        base_url: &str,
        cookie: &str,
    ) -> Result<(Metadata, R), Error>;
    async fn do_status_request(
        url: &str,
        method: Method,
        base_url: &str,
        cookie: &str,
    ) -> Result<Metadata, Error>;
    async fn do_string_string_request<const JSON: bool>(
        url: &str,
        method: Method,
        body: String,
        base_url: &str,
        cookie: &str,
    ) -> Result<(Metadata, String), Error>;
    async fn do_json_status_request<T: Serialize>(
        url: &str,
        method: Method,
        body: &T,
        base_url: &str,
        cookie: &str,
    ) -> Result<Metadata, Error>;
}
