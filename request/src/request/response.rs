use std::collections::HashMap;

use reqwest::StatusCode as RequestStatusCode;
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::Error;
pub type ResponseResult<T> = Result<Response<T>, Error>;

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(target_arch = "wasm32", derive(tsify_next::Tsify), tsify(into_wasm_abi, from_wasm_abi))]
///The response with a generic body
pub struct Response<B> {
    metadata: Metadata,
    body: B,
}
impl<B> Response<B> {
    pub fn map_body<U, F: FnOnce(B) -> U>(self, f: F) -> Response<U> {
        Response { metadata: self.metadata, body: f(self.body) }
    }
    pub fn try_map_body<U, E, F: FnOnce(B) -> Result<U, E>>(self, f: F) -> Result<Response<U>, E> {
        Ok(Response { metadata: self.metadata, body: f(self.body)? })
    }
}
impl<B, T: Into<Metadata>> From<(T, B)> for Response<B> {
    fn from((metadata, body): (T, B)) -> Self {
        Self { metadata: metadata.into(), body }
    }
}
impl<T: Into<Metadata>> From<T> for Response<()> {
    fn from(metadata: T) -> Self {
        Self { metadata: metadata.into(), body: () }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
#[cfg_attr(target_arch = "wasm32", derive(tsify_next::Tsify), tsify(into_wasm_abi, from_wasm_abi))]
/// The statuscode of the result
pub struct StatusCode(u16);
impl From<RequestStatusCode> for StatusCode {
    fn from(value: RequestStatusCode) -> Self {
        StatusCode(unsafe { u16::from(value).try_into().unwrap_unchecked() })
    }
}
/// A struct to cary the intermidiary of the metadata between the frontend and backend
#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(target_arch = "wasm32", derive(tsify_next::Tsify), tsify(into_wasm_abi, from_wasm_abi))]
/// The headers and statuscode of the response
pub struct Metadata {
    status: StatusCode,
    headers: HashMap<Option<String>, Result<String, String>>,
}
impl Metadata {
    #[allow(unused)]
    fn new(map: HeaderMap, status: RequestStatusCode) -> Self {
        Self {
            status: status.into(),
            headers: map
                .into_iter()
                .map(|(v, e)| {
                    (
                        v.map(|e| e.as_str().to_string()),
                        String::from_utf8(e.as_bytes().to_vec()).map_err(|e| e.to_string()),
                    )
                })
                .collect(),
        }
    }
}
impl From<(RequestStatusCode, HeaderMap)> for Metadata {
    fn from((status, headermap): (RequestStatusCode, HeaderMap)) -> Self {
        Self {
            headers: headermap
                .into_iter()
                .map(|(v, e)| {
                    (
                        v.map(|e| e.as_str().to_string()),
                        String::from_utf8(e.as_bytes().to_vec()).map_err(|e| e.to_string()),
                    )
                })
                .collect(),
            status: status.into(),
        }
    }
}
impl From<(HeaderMap, RequestStatusCode)> for Metadata {
    fn from((headermap, status): (HeaderMap, RequestStatusCode)) -> Self {
        Self {
            headers: headermap
                .into_iter()
                .map(|(v, e)| {
                    (
                        v.map(|e| e.as_str().to_string()),
                        String::from_utf8(e.as_bytes().to_vec()).map_err(|e| e.to_string()),
                    )
                })
                .collect(),
            status: status.into(),
        }
    }
}
impl From<(&HeaderMap, RequestStatusCode)> for Metadata {
    fn from((headermap, status): (&HeaderMap, RequestStatusCode)) -> Self {
        Self {
            headers: headermap
                .into_iter()
                .map(|(v, e)| {
                    (
                        Some(v.as_str().to_string()),
                        String::from_utf8(e.as_bytes().to_vec()).map_err(|e| e.to_string()),
                    )
                })
                .collect(),
            status: status.into(),
        }
    }
}
impl From<(RequestStatusCode, &HeaderMap)> for Metadata {
    fn from((status, headermap): (RequestStatusCode, &HeaderMap)) -> Self {
        Self {
            status: status.into(),
            headers: headermap
                .into_iter()
                .map(|(v, e)| {
                    (
                        Some(v.as_str().to_string()),
                        String::from_utf8(e.as_bytes().to_vec()).map_err(|e| e.to_string()),
                    )
                })
                .collect(),
        }
    }
}
