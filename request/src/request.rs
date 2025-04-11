use reqwest::Method;
pub use response::*;
use serde::Serialize;
use serde::de::DeserializeOwned;
pub const JSON_CONTENT_TYPE: &str = "application/json";
mod native;
pub mod response;
mod tauri;
#[cfg(feature = "tauri")]
pub use tauri::*;
#[cfg(all(feature = "tauri", target_arch = "wasm32"))]
pub type DefaultBackend = tauri::WasmTauri;

#[cfg(all(feature = "tauri", not(target_arch = "wasm32")))]
pub type DefaultBackend = tauri::Axum;

#[cfg(not(feature = "tauri"))]
pub type DefaultBackend = native::Native;

pub trait RequestBackend {
    #[cfg(all(feature = "tauri", target_arch = "wasm32"))]
    const DEFAULT: tauri::WasmTauri = tauri::WasmTauri;
    #[cfg(all(feature = "tauri", not(target_arch = "wasm32")))]
    const DEFAULT: tauri::Axum = tauri::Axum;
    #[cfg(not(feature = "tauri"))]
    const DEFAULT: native::Native = native::Native;
    async fn do_json_json_request<T: Serialize, R: DeserializeOwned>(
        url: &str,
        method: Method,
        body: &T,
        base_url: &str,
        cookie: &str,
    ) -> ResponseResult<R>;
    async fn do_string_json_request<const JSON: bool, R: DeserializeOwned>(
        url: &str,
        method: Method,
        body: String,
        base_url: &str,
        cookie: &str,
    ) -> ResponseResult<R>;
    async fn do_status_request(
        url: &str,
        method: Method,
        base_url: &str,
        cookie: &str,
    ) -> ResponseResult<()>;
    async fn do_string_string_request<const JSON: bool>(
        url: &str,
        method: Method,
        body: String,
        base_url: &str,
        cookie: &str,
    ) -> ResponseResult<String>;
    async fn do_json_status_request<T: Serialize>(
        url: &str,
        method: Method,
        body: &T,
        base_url: &str,
        cookie: &str,
    ) -> ResponseResult<()>;
}
