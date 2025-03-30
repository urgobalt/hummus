mod communication;
mod error;
use error::Error;
use reqwest::{Client, Method, StatusCode};
use serde::{Deserialize, Serialize};
#[derive(Serialize, PartialEq, Deserialize)]
pub struct Note {
    text: String,
}
#[derive(Serialize, PartialEq, Deserialize)]
pub struct NotsssseUpdate {
    note: Note,
    note_id: NoteID,
}
#[derive(Serialize, PartialEq, Deserialize)]
pub struct StoreID(pub String);
pub const DEFAULT_HOST: &'static str = "localhost:3000";
#[derive(Serialize, PartialEq, Deserialize)]
pub struct NoteID(pub u64);
#[derive(Serialize, PartialEq, Deserialize)]
pub struct UserID(pub u64);

#[derive(Serialize, PartialEq, Deserialize)]
pub struct StoreSession(pub u64);
#[derive(Serialize, PartialEq, Deserialize)]
pub struct IDSSession(pub u64);

#[derive(Serialize, PartialEq, Deserialize, Clone)]
pub struct ServerState {}
pub(crate) use internal::*;
pub mod tauri_axum {
    use super::*;
    use axum::Router;
    use axum::http::{Request, Response};
    use reqwest::header::COOKIE;
    use serde::de::DeserializeOwned;
    use std::sync::{Mutex, OnceLock};
    use tower_service::Service;

    static SERVER: OnceLock<Mutex<axum::Router<()>>> = OnceLock::new();
    /// remeber to call with_state with the correct ServerState lol
    pub fn init_with_axum_server(router: Router) {
        SERVER
            .set(Mutex::new(router))
            .expect("Server not to be initalized before")
    }
    async fn do_request_from_tauri(
        local_path: &'static str,
        method: Method,
        body: String,
        cookie: &str,
        backing_url: &str,
    ) -> Result<Response<axum::body::Body>, Error> {
        let request = Request::builder()
            .method(method)
            .uri(local_path)
            .header("BACKING_URL", backing_url)
            .header(COOKIE, cookie)
            .body(body)?;
        let mutex = SERVER.get().expect("Server to be started");
        let mut server = mutex.lock().expect("poisoned");
        Ok(server.call(request).await.expect("infalible"))
    }

    pub(crate) async fn do_json_json_request<T: Serialize, R: DeserializeOwned>(
        url: &'static str,
        method: Method,
        body: &T,
        backing_url: &str,
        cookie: &str,
    ) -> Result<(StatusCode, R), Error> {
        let resp = do_request_from_tauri(
            url,
            method,
            serde_json::to_string(body)?,
            cookie,
            backing_url,
        )
        .await?;
        let a = resp.body().clone();
        Ok((resp.status(),))
    }
    #[inline(always)]
    pub(crate) async fn do_string_json_request<R: DeserializeOwned>(
        url: &'static str,
        method: Method,
        body: String,
        base_url: &str,
        cookie: &str,
    ) -> Result<(StatusCode, R), Error> {
        todo!()
    }
    #[inline(always)]
    pub(crate) async fn do_json_status_request<T: Serialize>(
        url: &'static str,
        method: Method,
        body: &T,
        base_url: &str,
        cookie: &str,
    ) -> Result<StatusCode, Error> {
        todo!()
    }
}
#[cfg(not(feature = "tauri_axum"))]
pub mod internal {
    use super::*;
    use reqwest::header::COOKIE;
    use serde::de::DeserializeOwned;
    #[inline(always)]
    pub(crate) async fn do_json_json_request<T: Serialize, R: DeserializeOwned>(
        url: &'static str,
        method: Method,
        body: &T,
        base_url: &str,
        cookie: &str,
    ) -> Result<(StatusCode, R), Error> {
        let resp = Client::new()
            .request(method, format!("{base_url}/{url}"))
            .header(COOKIE, cookie)
            .json(body)
            .send()
            .await?;
        Ok((resp.status(), resp.json().await?))
    }
    #[inline(always)]
    pub(crate) async fn do_string_json_request<R: DeserializeOwned>(
        url: &'static str,
        method: Method,
        body: String,
        base_url: &str,
        cookie: &str,
    ) -> Result<(StatusCode, R), Error> {
        let resp = Client::new()
            .request(method, format!("{base_url}/{url}"))
            .header(COOKIE, cookie)
            .body(body)
            .send()
            .await?;
        Ok((resp.status(), resp.json().await?))
    }
    #[inline(always)]
    pub(crate) async fn do_json_status_request<T: Serialize>(
        url: &'static str,
        method: Method,
        body: &T,
        base_url: &str,
        cookie: &str,
    ) -> Result<StatusCode, Error> {
        let resp = Client::new()
            .request(method, format!("{base_url}/{url}"))
            .header(COOKIE, cookie)
            .json(body)
            .send()
            .await?;
        Ok(resp.status())
    }
    pub fn ensure_valid_base_url(url: &str) -> &str {
        url.trim_end_matches('/')
    }
}
pub mod note {
    use super::*;
    pub async fn add_new_note(note: &Note, store: &str, cookie: &str) -> Result<StatusCode, Error> {
        do_json_status_request("api/note/new", Method::PUT, note, store, cookie).await
    }
    pub async fn update_note(
        note: &NoteUpdate,
        store: &str,
        cookie: &str,
    ) -> Result<(StatusCode, Option<Note>), Error> {
        do_json_json_request("api/note/update", Method::PATCH, note, store, cookie).await
    }
    pub async fn get_note_by_id(
        note: &NoteID,
        store: &str,
        cookie: &str,
    ) -> Result<(StatusCode, Option<Note>), Error> {
        do_json_json_request("api/note/update", Method::PATCH, note, store, cookie).await
    }
}
mod ids {}
mod to_local {}
