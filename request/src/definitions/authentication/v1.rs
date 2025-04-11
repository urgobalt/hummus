use serde::{Deserialize, Serialize};

use crate::definitions::APIV1;

use super::Sealed;

#[derive(Serialize, PartialEq, Deserialize, Clone)]
#[cfg_attr(target_arch = "wasm32", derive(tsify_next::Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct SessionV1 {
    cookie: String,
    store_url: String,
    ids: String,
}
impl Sealed for SessionV1 {}
impl APIV1 for SessionV1 {}
impl SessionV1 {
    pub fn get_cookie(&self) -> &str {
        &self.cookie
    }
    pub fn get_store_url(&self) -> &str {
        &self.store_url
    }
    pub fn get_ids_url(&self) -> &str {
        &self.ids
    }
}
