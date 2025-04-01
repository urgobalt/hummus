use super::{JSON_CONTENT_TYPE, RequestBackend};
use crate::error::Error;
use reqwest::{Method, StatusCode, header::COOKIE};
use serde::Deserialize;
use serde::{Serialize, de::DeserializeOwned};
use std::num::NonZeroU16;
#[derive(Serialize, Deserialize)]
struct StatusCodeIntermediary(NonZeroU16);
impl From<StatusCode> for StatusCodeIntermediary {
    fn from(value: StatusCode) -> Self {
        // Safety: The backing implementaion is a NonZeroU16 aswell
        // it is less safe for it to be a u16 that would allow the correct translation beacuse then
        // smth could shange the internal status of the StatusCodeIntermediary
        StatusCodeIntermediary(unsafe { u16::from(value).try_into().unwrap_unchecked() })
    }
}
impl Into<StatusCode> for StatusCodeIntermediary {
    fn into(self) -> StatusCode {
        StatusCode::from_u16(self.0.into()).expect("Invalid code saved")
    }
}

#[derive(Serialize)]
pub struct StatusRequest<'a> {
    url: &'static str,
    method: &'a str,
    base_url: &'a str,
    cookie: &'a str,
}
#[derive(Serialize)]
pub struct StringJsonRequest<'a> {
    url: &'static str,
    method: &'a str,
    body: String,
    base_url: &'a str,
    cookie: &'a str,
}
#[derive(Serialize)]
pub struct JsonJsonRequest<'a> {
    url: &'static str,
    method: &'a str,
    body: String,
    base_url: &'a str,
    cookie: &'a str,
}
#[derive(Serialize)]
pub struct JsonStatusRequest<'a> {
    url: &'static str,
    method: &'a str,
    body: String,
    base_url: &'a str,
    cookie: &'a str,
}
#[derive(Serialize)]
pub struct StringStringRequest<'a> {
    url: &'static str,
    method: &'a str,
    body: String,
    base_url: &'a str,
    cookie: &'a str,
}

#[cfg(all(feature = "tauri-wasm", target_arch = "wasm32"))]
pub type Tauri = WasmTauri;
#[cfg(all(feature = "tauri-wasm", target_arch = "wasm32"))]
pub struct WasmTauri;
#[cfg(all(feature = "tauri-wasm", target_arch = "wasm32"))]
mod tauri_binding {
    use super::*;
    use crate::request::RequestBackend;
    use crate::request::wasm::JsonJsonRequest;
    use tauri_wasm::Data;
    use tauri_wasm::{invoke_with_args, is_tauri};

    impl RequestBackend for WasmTauri {
        async fn do_json_json_request<T: serde::Serialize, R: serde::de::DeserializeOwned>(
            url: &'static str,
            method: reqwest::Method,
            body: &T,
            base_url: &str,
            cookie: &str,
        ) -> Result<(reqwest::StatusCode, R), crate::error::Error> {
            assert!(
                is_tauri(),
                "To be running in a tauri enviroment based on the compile flags and RequestBackend used"
            );
            let js_value = invoke_with_args(
                "tauri_json_json_request",
                Data(JsonJsonRequest {
                    url,
                    body: serde_json::to_string(body)?,
                    method: method.as_str(),
                    base_url,
                    cookie,
                }),
            )
            .await?;
            let (s, value) = js_value_to_status_string(js_value)?;
            let converted = serde_json::from_str(&value)?;
            Ok((s, converted))
        }

        async fn do_string_json_request<const JSON: bool, R: serde::de::DeserializeOwned>(
            url: &'static str,
            method: reqwest::Method,
            body: String,
            base_url: &str,
            cookie: &str,
        ) -> Result<(reqwest::StatusCode, R), crate::error::Error> {
            assert!(
                is_tauri(),
                "To be running in a tauri enviroment based on the compile flags and RequestBackend used"
            );
            let js_value = match JSON {
                true => {
                    invoke_with_args(
                        "tauri_string_json_request_json",
                        Data(StringJsonRequest {
                            url,
                            body,
                            method: method.as_str(),
                            base_url,
                            cookie,
                        }),
                    )
                    .await?
                }
                false => {
                    invoke_with_args(
                        "tauri_string_json_request_no_json",
                        Data(StringJsonRequest {
                            url,
                            body,
                            method: method.as_str(),
                            base_url,
                            cookie,
                        }),
                    )
                    .await?
                }
            };
            let (s, value) = js_value_to_status_string(js_value)?;
            let converted = serde_json::from_str(&value)?;
            Ok((s, converted))
        }

        async fn do_status_request(
            url: &'static str,
            method: reqwest::Method,
            base_url: &str,
            cookie: &str,
        ) -> Result<reqwest::StatusCode, crate::error::Error> {
            assert!(
                is_tauri(),
                "To be running in a tauri enviroment based on the compile flags and RequestBackend used"
            );
            let js_value = invoke_with_args(
                "tauri_status_request",
                Data(StatusRequest {
                    url,
                    method: method.as_str(),
                    base_url,
                    cookie,
                }),
            )
            .await?;

            js_value_to_status(js_value)
        }

        async fn do_json_status_request<T: serde::Serialize>(
            url: &'static str,
            method: reqwest::Method,
            body: &T,
            base_url: &str,
            cookie: &str,
        ) -> Result<reqwest::StatusCode, crate::error::Error> {
            assert!(
                is_tauri(),
                "To be running in a tauri enviroment based on the compile flags and RequestBackend used"
            );
            let js_value = invoke_with_args(
                "tauri_json_status_request",
                Data(StatusRequest {
                    url,
                    method: method.as_str(),
                    base_url,
                    cookie,
                }),
            )
            .await?;

            js_value_to_status(js_value)
        }

        async fn do_string_string_request<const JSON: bool>(
            url: &'static str,
            method: Method,
            body: String,
            base_url: &str,
            cookie: &str,
        ) -> Result<(StatusCode, String), Error> {
            assert!(
                is_tauri(),
                "To be running in a tauri enviroment based on the compile flags and RequestBackend used"
            );
            let js_value = match JSON {
                true => {
                    invoke_with_args(
                        "tauri_string_string_request_json",
                        Data(StringStringRequest {
                            url,
                            method: method.as_str(),
                            body,
                            base_url,
                            cookie,
                        }),
                    )
                    .await?
                }
                false => {
                    invoke_with_args(
                        "tauri_string_string_request_no_json",
                        Data(StringStringRequest {
                            url,
                            method: method.as_str(),
                            body,
                            base_url,
                            cookie,
                        }),
                    )
                    .await?
                }
            };
            js_value_to_status_string(js_value)
        }
    }
    use wasm_bindgen::JsValue;
    fn js_value_to_status(js_value: JsValue) -> Result<StatusCode, Error> {
        let status: StatusCodeIntermediary = serde_wasm_bindgen::from_value(js_value)?;
        Ok(status.into())
    }
    fn js_value_to_status_string(js_value: JsValue) -> Result<(StatusCode, String), Error> {
        let (status, value): (StatusCodeIntermediary, String) =
            serde_wasm_bindgen::from_value(js_value)?;
        Ok((status.into(), value))
    }
}
#[cfg(all(feature = "tauri", not(target_arch = "wasm32")))]
pub type Tauri = Axum;
#[cfg(all(feature = "tauri", not(target_arch = "wasm32")))]
pub struct Axum;
#[cfg(all(feature = "tauri", not(target_arch = "wasm32")))]
mod axum_request {
    use std::sync::OnceLock;

    use axum::Router;
    use axum::body::{Body, to_bytes};
    use axum::http::{HeaderValue, Request, Response};
    use reqwest::header::CONTENT_TYPE;
    use std::sync::Mutex;
    use tower_service::Service;

    use crate::request;

    use super::*;
    impl RequestBackend for Axum {
        async fn do_json_json_request<T: Serialize, R: DeserializeOwned>(
            url: &'static str,
            method: Method,
            body: &T,
            backing_url: &str,
            cookie: &str,
        ) -> Result<(StatusCode, R), Error> {
            let resp = do_request_from_tauri::<true>(
                url,
                method,
                serde_json::to_string(body)?,
                cookie,
                backing_url,
            )
            .await?;
            let status = resp.status();
            let body = resp.into_body();
            let data = to_bytes(body, 1_000_000).await?;
            let val = serde_json::from_slice(&data)?;
            Ok((status, val))
        }
        #[inline(always)]
        async fn do_string_json_request<const JSON: bool, R: DeserializeOwned>(
            url: &'static str,
            method: Method,
            body: String,
            base_url: &str,
            cookie: &str,
        ) -> Result<(StatusCode, R), Error> {
            let resp = do_request_from_tauri::<JSON>(url, method, body, cookie, base_url).await?;
            let status = resp.status();
            let body = resp.into_body();
            let data = to_bytes(body, 1_000_000).await?;
            let val = serde_json::from_slice(&data)?;
            Ok((status, val))
        }
        #[inline(always)]
        async fn do_json_status_request<T: Serialize>(
            url: &'static str,
            method: Method,
            body: &T,
            base_url: &str,
            cookie: &str,
        ) -> Result<StatusCode, Error> {
            let resp = do_request_from_tauri::<true>(
                url,
                method,
                serde_json::to_string(body)?,
                cookie,
                base_url,
            )
            .await?;
            let status = resp.status();
            Ok(status)
        }

        async fn do_status_request(
            url: &'static str,
            method: Method,
            base_url: &str,
            cookie: &str,
        ) -> Result<StatusCode, Error> {
            let resp = do_request_from_tauri::<false>(url, method, "".to_owned(), cookie, base_url)
                .await?;
            let status = resp.status();
            Ok(status)
        }

        async fn do_string_string_request<const JSON: bool>(
            url: &'static str,
            method: Method,
            body: String,
            base_url: &str,
            cookie: &str,
        ) -> Result<(StatusCode, String), Error> {
            let resp = do_request_from_tauri::<JSON>(url, method, body, cookie, base_url).await?;
            let status = resp.status();
            let body = resp.into_body();
            let data = to_bytes(body, 1_000_000).await?;
            Ok((status, String::from_utf8_lossy(&data).to_string()))
        }
    }
    #[tauri::command]
    pub async fn tauri_json_json_request<'a>(
        value: JsonJsonRequest<'a>,
    ) -> Result<(StatusCode, String), Error> {
        Axum::do_string_string_request::<true>(
            value.url,
            value
                .method
                .parse()
                .expect("This should only be created from Method"),
            value.body,
            &value.base_url,
            &value.cookie,
        )
        .await
    }
    #[tauri::command]
    async fn tauri_string_json_request_json<'a>(
        value: StringJsonRequest<'a>,
    ) -> Result<(StatusCodeIntermediary, String), Error> {
        Axum::do_string_string_request::<true>(
            value.url,
            value
                .method
                .parse()
                .expect("This should only be created from Method"),
            value.body,
            &value.base_url,
            &value.cookie,
        )
        .await
        .map(convert_to_status_code)
    }
    #[tauri::command]
    async fn tauri_string_json_request_no_json<'a>(
        value: StringJsonRequest<'a>,
    ) -> Result<(StatusCodeIntermediary, String), Error> {
        Axum::do_string_string_request::<false>(
            value.url,
            value
                .method
                .parse()
                .expect("This should only be created from Method"),
            value.body,
            &value.base_url,
            &value.cookie,
        )
        .await
        .map(convert_to_status_code)
    }
    #[tauri::command]
    pub async fn tauri_json_status_request<'a>(
        value: JsonStatusRequest<'a>,
    ) -> Result<StatusCodeIntermediary, Error> {
        Axum::do_string_json_request::<true, ()>(
            value.url,
            value
                .method
                .parse()
                .expect("This should only be created from Method"),
            value.body,
            &value.base_url,
            &value.cookie,
        )
        .await
        .map(convert_to_status_code)
        .map(|(s, ())| s)
    }
    #[tauri::command]
    pub async fn tauri_status_request<'a>(
        value: StatusRequest<'a>,
    ) -> Result<StatusCodeIntermediary, Error> {
        Axum::do_status_request(
            value.url,
            value
                .method
                .parse()
                .expect("This should only be created from Method"),
            &value.base_url,
            &value.cookie,
        )
        .await
        .map(|v| v.into())
    }
    #[tauri::command]
    async fn tauri_string_string_request_json<'a>(
        value: StringStringRequest<'a>,
    ) -> Result<(StatusCodeIntermediary, String), Error> {
        Axum::do_string_string_request::<true>(
            value.url,
            value
                .method
                .parse()
                .expect("This should only be created from Method"),
            value.body,
            value.base_url,
            value.cookie,
        )
        .await
        .map(convert_to_status_code)
    }
    #[tauri::command]
    async fn tauri_string_string_request_no_json<'a>(
        value: StringStringRequest<'a>,
    ) -> Result<(StatusCodeIntermediary, String), Error> {
        Axum::do_string_string_request::<false>(
            value.url,
            value
                .method
                .parse()
                .expect("This should only be created from Method"),
            value.body,
            value.base_url,
            value.cookie,
        )
        .await
        .map(convert_to_status_code)
    }
    fn convert_to_status_code<T>(value: (StatusCode, T)) -> (StatusCodeIntermediary, T) {
        (value.0.into(), value.1)
    }
    static SERVER: OnceLock<Mutex<Router<()>>> = OnceLock::new();
    /// remeber to call with_state with the correct ServerState lol
    pub fn init_with_axum_server(router: Router) {
        SERVER
            .set(Mutex::new(router))
            .expect("Server not to be initalized before")
    }

    async fn do_request_from_tauri<const JSON: bool>(
        local_path: &'static str,
        method: Method,
        body: String,
        cookie: &str,
        backing_url: &str,
    ) -> Result<Response<Body>, Error> {
        let mut request = Request::builder()
            .method(method)
            .uri(local_path)
            .header("BACKING_URL", backing_url)
            .header(COOKIE, cookie);
        if JSON {
            request = request.header(CONTENT_TYPE, JSON_CONTENT_TYPE)
        }
        let request = request.body(body)?;
        let mutex = SERVER.get().expect("Server to be started");
        let mut server = mutex.lock().expect("poisoned");
        Ok(server.call(request).await.expect("infalible"))
    }
}
