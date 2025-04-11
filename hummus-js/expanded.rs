#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
pub mod note {
    use hummus_macros::specialize_function_for_bindgen;
    use hummus_request::definitions::authentication::SessionVersion;
    use hummus_request::definitions::note::{
        NoteIDVersion, NoteSearchVersion, NoteUpdateVersion, NoteVersion,
    };
    use hummus_request::{DefaultBackend, ResponseResult};
    #[allow(dead_code)]
    pub async fn get_note(
        note: NoteIDVersion,
        session: SessionVersion,
    ) -> ResponseResult<NoteVersion> {
        (hummus_request::api::note::get_note::<DefaultBackend>(note, &session))
            .await
            .map(|value| value.into())
    }
    #[automatically_derived]
    const _: () = {
        #[export_name = "get_note"]
        pub unsafe extern "C" fn __wasm_bindgen_generated_get_note(
            arg0_1: <<NoteIDVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim1,
            arg0_2: <<NoteIDVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim2,
            arg0_3: <<NoteIDVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim3,
            arg0_4: <<NoteIDVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim4,
            arg1_1: <<SessionVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim1,
            arg1_2: <<SessionVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim2,
            arg1_3: <<SessionVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim3,
            arg1_4: <<SessionVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim4,
        ) -> wasm_bindgen::convert::WasmRet<
            <wasm_bindgen::JsValue as wasm_bindgen::convert::ReturnWasmAbi>::Abi,
        > {
            const _: () = {};
            let _ret = wasm_bindgen_futures::future_to_promise(async move {
                    {
                        let arg0 = unsafe {
                            <NoteIDVersion as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                                <<NoteIDVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::join(
                                    arg0_1,
                                    arg0_2,
                                    arg0_3,
                                    arg0_4,
                                ),
                            )
                        };
                        let arg1 = unsafe {
                            <SessionVersion as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                                <<SessionVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::join(
                                    arg1_1,
                                    arg1_2,
                                    arg1_3,
                                    arg1_4,
                                ),
                            )
                        };
                        let _ret = get_note(arg0, arg1);
                        <ResponseResult<
                            NoteVersion,
                        > as wasm_bindgen::__rt::IntoJsResult>::into_js_result(
                            _ret.await,
                        )
                    }
                })
                .into();
            <wasm_bindgen::JsValue as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(
                    _ret,
                )
                .into()
        }
    };
    #[cfg(all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none")))]
    #[automatically_derived]
    const _: () = {
        #[no_mangle]
        #[doc(hidden)]
        pub extern "C" fn __wbindgen_describe_get_note() {
            use wasm_bindgen::describe::*;
            wasm_bindgen::__rt::link_mem_intrinsics();
            inform(FUNCTION);
            inform(0);
            inform(2u32);
            <NoteIDVersion as WasmDescribe>::describe();
            <SessionVersion as WasmDescribe>::describe();
            <wasm_bindgen::JsValue as WasmDescribe>::describe();
            <ResponseResult<NoteVersion> as WasmDescribe>::describe();
        }
    };
    #[cfg(all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none")))]
    #[automatically_derived]
    const _: () = {
        use wasm_bindgen::__rt::{flat_len, flat_byte_slices};
        static _INCLUDED_FILES: &[&str] = &[];
        const _ENCODED_BYTES: &[u8] = {
            const _CHUNK_SLICES: [&[u8]; 1usize] = [
                b"\x01\0\0\0\x02\x04note\0\0\x07session\0\0\x01\x08get_note\x01\x01\0\0\0\x01\x01\0\0\0\0\0\0\0\0\x1chummus-wasm-1d57e068314a5fcb\0\0",
            ];
            #[allow(long_running_const_eval)]
            const _CHUNK_LEN: usize = flat_len(_CHUNK_SLICES);
            #[allow(long_running_const_eval)]
            const _CHUNKS: [u8; _CHUNK_LEN] = flat_byte_slices(_CHUNK_SLICES);
            const _LEN_BYTES: [u8; 4] = (_CHUNK_LEN as u32).to_le_bytes();
            const _ENCODED_BYTES_LEN: usize = _CHUNK_LEN + 4;
            #[allow(long_running_const_eval)]
            const _ENCODED_BYTES: [u8; _ENCODED_BYTES_LEN] = flat_byte_slices([
                &_LEN_BYTES,
                &_CHUNKS,
            ]);
            &_ENCODED_BYTES
        };
        const _PREFIX_JSON_BYTES: &[u8] = b"0\0\0\0{\"schema_version\":\"0.2.100\",\"version\":\"0.2.100\"}";
        const _ENCODED_BYTES_LEN: usize = _ENCODED_BYTES.len();
        const _PREFIX_JSON_BYTES_LEN: usize = _PREFIX_JSON_BYTES.len();
        const _LEN: usize = _PREFIX_JSON_BYTES_LEN + _ENCODED_BYTES_LEN;
        #[link_section = "__wasm_bindgen_unstable"]
        #[allow(long_running_const_eval)]
        static _GENERATED: [u8; _LEN] = flat_byte_slices([
            _PREFIX_JSON_BYTES,
            _ENCODED_BYTES,
        ]);
    };
    #[allow(dead_code)]
    pub async fn add_new_note(
        note: NoteVersion,
        session: SessionVersion,
    ) -> ResponseResult<()> {
        (hummus_request::api::note::add_new_note::<DefaultBackend>(&note, &session))
            .await
            .map(|value| value.into())
    }
    #[automatically_derived]
    const _: () = {
        #[export_name = "add_new_note"]
        pub unsafe extern "C" fn __wasm_bindgen_generated_add_new_note(
            arg0_1: <<NoteVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim1,
            arg0_2: <<NoteVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim2,
            arg0_3: <<NoteVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim3,
            arg0_4: <<NoteVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim4,
            arg1_1: <<SessionVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim1,
            arg1_2: <<SessionVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim2,
            arg1_3: <<SessionVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim3,
            arg1_4: <<SessionVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim4,
        ) -> wasm_bindgen::convert::WasmRet<
            <wasm_bindgen::JsValue as wasm_bindgen::convert::ReturnWasmAbi>::Abi,
        > {
            const _: () = {};
            let _ret = wasm_bindgen_futures::future_to_promise(async move {
                    {
                        let arg0 = unsafe {
                            <NoteVersion as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                                <<NoteVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::join(
                                    arg0_1,
                                    arg0_2,
                                    arg0_3,
                                    arg0_4,
                                ),
                            )
                        };
                        let arg1 = unsafe {
                            <SessionVersion as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                                <<SessionVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::join(
                                    arg1_1,
                                    arg1_2,
                                    arg1_3,
                                    arg1_4,
                                ),
                            )
                        };
                        let _ret = add_new_note(arg0, arg1);
                        <ResponseResult<
                            (),
                        > as wasm_bindgen::__rt::IntoJsResult>::into_js_result(
                            _ret.await,
                        )
                    }
                })
                .into();
            <wasm_bindgen::JsValue as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(
                    _ret,
                )
                .into()
        }
    };
    #[cfg(all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none")))]
    #[automatically_derived]
    const _: () = {
        #[no_mangle]
        #[doc(hidden)]
        pub extern "C" fn __wbindgen_describe_add_new_note() {
            use wasm_bindgen::describe::*;
            wasm_bindgen::__rt::link_mem_intrinsics();
            inform(FUNCTION);
            inform(0);
            inform(2u32);
            <NoteVersion as WasmDescribe>::describe();
            <SessionVersion as WasmDescribe>::describe();
            <wasm_bindgen::JsValue as WasmDescribe>::describe();
            <ResponseResult<()> as WasmDescribe>::describe();
        }
    };
    #[cfg(all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none")))]
    #[automatically_derived]
    const _: () = {
        use wasm_bindgen::__rt::{flat_len, flat_byte_slices};
        static _INCLUDED_FILES: &[&str] = &[];
        const _ENCODED_BYTES: &[u8] = {
            const _CHUNK_SLICES: [&[u8]; 1usize] = [
                b"\x01\0\0\0\x02\x04note\0\0\x07session\0\0\x01\x0cadd_new_note\x01\x01\0\0\0\x01\x01\0\0\0\0\0\0\0\0\x1chummus-wasm-1d57e068314a5fcb\0\0",
            ];
            #[allow(long_running_const_eval)]
            const _CHUNK_LEN: usize = flat_len(_CHUNK_SLICES);
            #[allow(long_running_const_eval)]
            const _CHUNKS: [u8; _CHUNK_LEN] = flat_byte_slices(_CHUNK_SLICES);
            const _LEN_BYTES: [u8; 4] = (_CHUNK_LEN as u32).to_le_bytes();
            const _ENCODED_BYTES_LEN: usize = _CHUNK_LEN + 4;
            #[allow(long_running_const_eval)]
            const _ENCODED_BYTES: [u8; _ENCODED_BYTES_LEN] = flat_byte_slices([
                &_LEN_BYTES,
                &_CHUNKS,
            ]);
            &_ENCODED_BYTES
        };
        const _PREFIX_JSON_BYTES: &[u8] = b"0\0\0\0{\"schema_version\":\"0.2.100\",\"version\":\"0.2.100\"}";
        const _ENCODED_BYTES_LEN: usize = _ENCODED_BYTES.len();
        const _PREFIX_JSON_BYTES_LEN: usize = _PREFIX_JSON_BYTES.len();
        const _LEN: usize = _PREFIX_JSON_BYTES_LEN + _ENCODED_BYTES_LEN;
        #[link_section = "__wasm_bindgen_unstable"]
        #[allow(long_running_const_eval)]
        static _GENERATED: [u8; _LEN] = flat_byte_slices([
            _PREFIX_JSON_BYTES,
            _ENCODED_BYTES,
        ]);
    };
    #[allow(dead_code)]
    pub async fn update_note(
        note: NoteUpdateVersion,
        session: SessionVersion,
    ) -> ResponseResult<Option<NoteVersion>> {
        (hummus_request::api::note::update_note::<DefaultBackend>(&note, &session))
            .await
            .map(|value| value.into())
    }
    #[automatically_derived]
    const _: () = {
        #[export_name = "update_note"]
        pub unsafe extern "C" fn __wasm_bindgen_generated_update_note(
            arg0_1: <<NoteUpdateVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim1,
            arg0_2: <<NoteUpdateVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim2,
            arg0_3: <<NoteUpdateVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim3,
            arg0_4: <<NoteUpdateVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim4,
            arg1_1: <<SessionVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim1,
            arg1_2: <<SessionVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim2,
            arg1_3: <<SessionVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim3,
            arg1_4: <<SessionVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim4,
        ) -> wasm_bindgen::convert::WasmRet<
            <wasm_bindgen::JsValue as wasm_bindgen::convert::ReturnWasmAbi>::Abi,
        > {
            const _: () = {};
            let _ret = wasm_bindgen_futures::future_to_promise(async move {
                    {
                        let arg0 = unsafe {
                            <NoteUpdateVersion as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                                <<NoteUpdateVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::join(
                                    arg0_1,
                                    arg0_2,
                                    arg0_3,
                                    arg0_4,
                                ),
                            )
                        };
                        let arg1 = unsafe {
                            <SessionVersion as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                                <<SessionVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::join(
                                    arg1_1,
                                    arg1_2,
                                    arg1_3,
                                    arg1_4,
                                ),
                            )
                        };
                        let _ret = update_note(arg0, arg1);
                        <ResponseResult<
                            Option<NoteVersion>,
                        > as wasm_bindgen::__rt::IntoJsResult>::into_js_result(
                            _ret.await,
                        )
                    }
                })
                .into();
            <wasm_bindgen::JsValue as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(
                    _ret,
                )
                .into()
        }
    };
    #[cfg(all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none")))]
    #[automatically_derived]
    const _: () = {
        #[no_mangle]
        #[doc(hidden)]
        pub extern "C" fn __wbindgen_describe_update_note() {
            use wasm_bindgen::describe::*;
            wasm_bindgen::__rt::link_mem_intrinsics();
            inform(FUNCTION);
            inform(0);
            inform(2u32);
            <NoteUpdateVersion as WasmDescribe>::describe();
            <SessionVersion as WasmDescribe>::describe();
            <wasm_bindgen::JsValue as WasmDescribe>::describe();
            <ResponseResult<Option<NoteVersion>> as WasmDescribe>::describe();
        }
    };
    #[cfg(all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none")))]
    #[automatically_derived]
    const _: () = {
        use wasm_bindgen::__rt::{flat_len, flat_byte_slices};
        static _INCLUDED_FILES: &[&str] = &[];
        const _ENCODED_BYTES: &[u8] = {
            const _CHUNK_SLICES: [&[u8]; 1usize] = [
                b"\x01\0\0\0\x02\x04note\0\0\x07session\0\0\x01\x0bupdate_note\x01\x01\0\0\0\x01\x01\0\0\0\0\0\0\0\0\x1chummus-wasm-1d57e068314a5fcb\0\0",
            ];
            #[allow(long_running_const_eval)]
            const _CHUNK_LEN: usize = flat_len(_CHUNK_SLICES);
            #[allow(long_running_const_eval)]
            const _CHUNKS: [u8; _CHUNK_LEN] = flat_byte_slices(_CHUNK_SLICES);
            const _LEN_BYTES: [u8; 4] = (_CHUNK_LEN as u32).to_le_bytes();
            const _ENCODED_BYTES_LEN: usize = _CHUNK_LEN + 4;
            #[allow(long_running_const_eval)]
            const _ENCODED_BYTES: [u8; _ENCODED_BYTES_LEN] = flat_byte_slices([
                &_LEN_BYTES,
                &_CHUNKS,
            ]);
            &_ENCODED_BYTES
        };
        const _PREFIX_JSON_BYTES: &[u8] = b"0\0\0\0{\"schema_version\":\"0.2.100\",\"version\":\"0.2.100\"}";
        const _ENCODED_BYTES_LEN: usize = _ENCODED_BYTES.len();
        const _PREFIX_JSON_BYTES_LEN: usize = _PREFIX_JSON_BYTES.len();
        const _LEN: usize = _PREFIX_JSON_BYTES_LEN + _ENCODED_BYTES_LEN;
        #[link_section = "__wasm_bindgen_unstable"]
        #[allow(long_running_const_eval)]
        static _GENERATED: [u8; _LEN] = flat_byte_slices([
            _PREFIX_JSON_BYTES,
            _ENCODED_BYTES,
        ]);
    };
    #[allow(dead_code)]
    pub async fn delete_note(
        note: NoteIDVersion,
        session: SessionVersion,
    ) -> ResponseResult<()> {
        (hummus_request::api::note::delete_note::<DefaultBackend>(&note, &session))
            .await
            .map(|value| value.into())
    }
    #[automatically_derived]
    const _: () = {
        #[export_name = "delete_note"]
        pub unsafe extern "C" fn __wasm_bindgen_generated_delete_note(
            arg0_1: <<NoteIDVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim1,
            arg0_2: <<NoteIDVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim2,
            arg0_3: <<NoteIDVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim3,
            arg0_4: <<NoteIDVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim4,
            arg1_1: <<SessionVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim1,
            arg1_2: <<SessionVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim2,
            arg1_3: <<SessionVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim3,
            arg1_4: <<SessionVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim4,
        ) -> wasm_bindgen::convert::WasmRet<
            <wasm_bindgen::JsValue as wasm_bindgen::convert::ReturnWasmAbi>::Abi,
        > {
            const _: () = {};
            let _ret = wasm_bindgen_futures::future_to_promise(async move {
                    {
                        let arg0 = unsafe {
                            <NoteIDVersion as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                                <<NoteIDVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::join(
                                    arg0_1,
                                    arg0_2,
                                    arg0_3,
                                    arg0_4,
                                ),
                            )
                        };
                        let arg1 = unsafe {
                            <SessionVersion as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                                <<SessionVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::join(
                                    arg1_1,
                                    arg1_2,
                                    arg1_3,
                                    arg1_4,
                                ),
                            )
                        };
                        let _ret = delete_note(arg0, arg1);
                        <ResponseResult<
                            (),
                        > as wasm_bindgen::__rt::IntoJsResult>::into_js_result(
                            _ret.await,
                        )
                    }
                })
                .into();
            <wasm_bindgen::JsValue as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(
                    _ret,
                )
                .into()
        }
    };
    #[cfg(all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none")))]
    #[automatically_derived]
    const _: () = {
        #[no_mangle]
        #[doc(hidden)]
        pub extern "C" fn __wbindgen_describe_delete_note() {
            use wasm_bindgen::describe::*;
            wasm_bindgen::__rt::link_mem_intrinsics();
            inform(FUNCTION);
            inform(0);
            inform(2u32);
            <NoteIDVersion as WasmDescribe>::describe();
            <SessionVersion as WasmDescribe>::describe();
            <wasm_bindgen::JsValue as WasmDescribe>::describe();
            <ResponseResult<()> as WasmDescribe>::describe();
        }
    };
    #[cfg(all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none")))]
    #[automatically_derived]
    const _: () = {
        use wasm_bindgen::__rt::{flat_len, flat_byte_slices};
        static _INCLUDED_FILES: &[&str] = &[];
        const _ENCODED_BYTES: &[u8] = {
            const _CHUNK_SLICES: [&[u8]; 1usize] = [
                b"\x01\0\0\0\x02\x04note\0\0\x07session\0\0\x01\x0bdelete_note\x01\x01\0\0\0\x01\x01\0\0\0\0\0\0\0\0\x1chummus-wasm-1d57e068314a5fcb\0\0",
            ];
            #[allow(long_running_const_eval)]
            const _CHUNK_LEN: usize = flat_len(_CHUNK_SLICES);
            #[allow(long_running_const_eval)]
            const _CHUNKS: [u8; _CHUNK_LEN] = flat_byte_slices(_CHUNK_SLICES);
            const _LEN_BYTES: [u8; 4] = (_CHUNK_LEN as u32).to_le_bytes();
            const _ENCODED_BYTES_LEN: usize = _CHUNK_LEN + 4;
            #[allow(long_running_const_eval)]
            const _ENCODED_BYTES: [u8; _ENCODED_BYTES_LEN] = flat_byte_slices([
                &_LEN_BYTES,
                &_CHUNKS,
            ]);
            &_ENCODED_BYTES
        };
        const _PREFIX_JSON_BYTES: &[u8] = b"0\0\0\0{\"schema_version\":\"0.2.100\",\"version\":\"0.2.100\"}";
        const _ENCODED_BYTES_LEN: usize = _ENCODED_BYTES.len();
        const _PREFIX_JSON_BYTES_LEN: usize = _PREFIX_JSON_BYTES.len();
        const _LEN: usize = _PREFIX_JSON_BYTES_LEN + _ENCODED_BYTES_LEN;
        #[link_section = "__wasm_bindgen_unstable"]
        #[allow(long_running_const_eval)]
        static _GENERATED: [u8; _LEN] = flat_byte_slices([
            _PREFIX_JSON_BYTES,
            _ENCODED_BYTES,
        ]);
    };
    #[allow(dead_code)]
    pub async fn search_notes(
        search: NoteSearchVersion,
        session: SessionVersion,
    ) -> ResponseResult<Vec<NoteIDVersion>> {
        (hummus_request::api::note::search_notes::<DefaultBackend>(&search, &session))
            .await
            .map(|value| value.into())
    }
    #[automatically_derived]
    const _: () = {
        #[export_name = "search_notes"]
        pub unsafe extern "C" fn __wasm_bindgen_generated_search_notes(
            arg0_1: <<NoteSearchVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim1,
            arg0_2: <<NoteSearchVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim2,
            arg0_3: <<NoteSearchVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim3,
            arg0_4: <<NoteSearchVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim4,
            arg1_1: <<SessionVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim1,
            arg1_2: <<SessionVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim2,
            arg1_3: <<SessionVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim3,
            arg1_4: <<SessionVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim4,
        ) -> wasm_bindgen::convert::WasmRet<
            <wasm_bindgen::JsValue as wasm_bindgen::convert::ReturnWasmAbi>::Abi,
        > {
            const _: () = {};
            let _ret = wasm_bindgen_futures::future_to_promise(async move {
                    {
                        let arg0 = unsafe {
                            <NoteSearchVersion as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                                <<NoteSearchVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::join(
                                    arg0_1,
                                    arg0_2,
                                    arg0_3,
                                    arg0_4,
                                ),
                            )
                        };
                        let arg1 = unsafe {
                            <SessionVersion as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                                <<SessionVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::join(
                                    arg1_1,
                                    arg1_2,
                                    arg1_3,
                                    arg1_4,
                                ),
                            )
                        };
                        let _ret = search_notes(arg0, arg1);
                        <ResponseResult<
                            Vec<NoteIDVersion>,
                        > as wasm_bindgen::__rt::IntoJsResult>::into_js_result(
                            _ret.await,
                        )
                    }
                })
                .into();
            <wasm_bindgen::JsValue as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(
                    _ret,
                )
                .into()
        }
    };
    #[cfg(all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none")))]
    #[automatically_derived]
    const _: () = {
        #[no_mangle]
        #[doc(hidden)]
        pub extern "C" fn __wbindgen_describe_search_notes() {
            use wasm_bindgen::describe::*;
            wasm_bindgen::__rt::link_mem_intrinsics();
            inform(FUNCTION);
            inform(0);
            inform(2u32);
            <NoteSearchVersion as WasmDescribe>::describe();
            <SessionVersion as WasmDescribe>::describe();
            <wasm_bindgen::JsValue as WasmDescribe>::describe();
            <ResponseResult<Vec<NoteIDVersion>> as WasmDescribe>::describe();
        }
    };
    #[cfg(all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none")))]
    #[automatically_derived]
    const _: () = {
        use wasm_bindgen::__rt::{flat_len, flat_byte_slices};
        static _INCLUDED_FILES: &[&str] = &[];
        const _ENCODED_BYTES: &[u8] = {
            const _CHUNK_SLICES: [&[u8]; 1usize] = [
                b"\x01\0\0\0\x02\x06search\0\0\x07session\0\0\x01\x0csearch_notes\x01\x01\0\0\0\x01\x01\0\0\0\0\0\0\0\0\x1chummus-wasm-1d57e068314a5fcb\0\0",
            ];
            #[allow(long_running_const_eval)]
            const _CHUNK_LEN: usize = flat_len(_CHUNK_SLICES);
            #[allow(long_running_const_eval)]
            const _CHUNKS: [u8; _CHUNK_LEN] = flat_byte_slices(_CHUNK_SLICES);
            const _LEN_BYTES: [u8; 4] = (_CHUNK_LEN as u32).to_le_bytes();
            const _ENCODED_BYTES_LEN: usize = _CHUNK_LEN + 4;
            #[allow(long_running_const_eval)]
            const _ENCODED_BYTES: [u8; _ENCODED_BYTES_LEN] = flat_byte_slices([
                &_LEN_BYTES,
                &_CHUNKS,
            ]);
            &_ENCODED_BYTES
        };
        const _PREFIX_JSON_BYTES: &[u8] = b"0\0\0\0{\"schema_version\":\"0.2.100\",\"version\":\"0.2.100\"}";
        const _ENCODED_BYTES_LEN: usize = _ENCODED_BYTES.len();
        const _PREFIX_JSON_BYTES_LEN: usize = _PREFIX_JSON_BYTES.len();
        const _LEN: usize = _PREFIX_JSON_BYTES_LEN + _ENCODED_BYTES_LEN;
        #[link_section = "__wasm_bindgen_unstable"]
        #[allow(long_running_const_eval)]
        static _GENERATED: [u8; _LEN] = flat_byte_slices([
            _PREFIX_JSON_BYTES,
            _ENCODED_BYTES,
        ]);
    };
    #[allow(dead_code)]
    pub async fn get_all_notes(
        session: SessionVersion,
    ) -> ResponseResult<Vec<NoteIDVersion>> {
        (hummus_request::api::note::get_all_notes::<DefaultBackend>(&session))
            .await
            .map(|value| value.into())
    }
    #[automatically_derived]
    const _: () = {
        #[export_name = "get_all_notes"]
        pub unsafe extern "C" fn __wasm_bindgen_generated_get_all_notes(
            arg0_1: <<SessionVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim1,
            arg0_2: <<SessionVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim2,
            arg0_3: <<SessionVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim3,
            arg0_4: <<SessionVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim4,
        ) -> wasm_bindgen::convert::WasmRet<
            <wasm_bindgen::JsValue as wasm_bindgen::convert::ReturnWasmAbi>::Abi,
        > {
            const _: () = {};
            let _ret = wasm_bindgen_futures::future_to_promise(async move {
                    {
                        let arg0 = unsafe {
                            <SessionVersion as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                                <<SessionVersion as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::join(
                                    arg0_1,
                                    arg0_2,
                                    arg0_3,
                                    arg0_4,
                                ),
                            )
                        };
                        let _ret = get_all_notes(arg0);
                        <ResponseResult<
                            Vec<NoteIDVersion>,
                        > as wasm_bindgen::__rt::IntoJsResult>::into_js_result(
                            _ret.await,
                        )
                    }
                })
                .into();
            <wasm_bindgen::JsValue as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(
                    _ret,
                )
                .into()
        }
    };
    #[cfg(all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none")))]
    #[automatically_derived]
    const _: () = {
        #[no_mangle]
        #[doc(hidden)]
        pub extern "C" fn __wbindgen_describe_get_all_notes() {
            use wasm_bindgen::describe::*;
            wasm_bindgen::__rt::link_mem_intrinsics();
            inform(FUNCTION);
            inform(0);
            inform(1u32);
            <SessionVersion as WasmDescribe>::describe();
            <wasm_bindgen::JsValue as WasmDescribe>::describe();
            <ResponseResult<Vec<NoteIDVersion>> as WasmDescribe>::describe();
        }
    };
    #[cfg(all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none")))]
    #[automatically_derived]
    const _: () = {
        use wasm_bindgen::__rt::{flat_len, flat_byte_slices};
        static _INCLUDED_FILES: &[&str] = &[];
        const _ENCODED_BYTES: &[u8] = {
            const _CHUNK_SLICES: [&[u8]; 1usize] = [
                b"\x01\0\0\0\x01\x07session\0\0\x01\rget_all_notes\x01\x01\0\0\0\x01\x01\0\0\0\0\0\0\0\0\x1chummus-wasm-1d57e068314a5fcb\0\0",
            ];
            #[allow(long_running_const_eval)]
            const _CHUNK_LEN: usize = flat_len(_CHUNK_SLICES);
            #[allow(long_running_const_eval)]
            const _CHUNKS: [u8; _CHUNK_LEN] = flat_byte_slices(_CHUNK_SLICES);
            const _LEN_BYTES: [u8; 4] = (_CHUNK_LEN as u32).to_le_bytes();
            const _ENCODED_BYTES_LEN: usize = _CHUNK_LEN + 4;
            #[allow(long_running_const_eval)]
            const _ENCODED_BYTES: [u8; _ENCODED_BYTES_LEN] = flat_byte_slices([
                &_LEN_BYTES,
                &_CHUNKS,
            ]);
            &_ENCODED_BYTES
        };
        const _PREFIX_JSON_BYTES: &[u8] = b"0\0\0\0{\"schema_version\":\"0.2.100\",\"version\":\"0.2.100\"}";
        const _ENCODED_BYTES_LEN: usize = _ENCODED_BYTES.len();
        const _PREFIX_JSON_BYTES_LEN: usize = _PREFIX_JSON_BYTES.len();
        const _LEN: usize = _PREFIX_JSON_BYTES_LEN + _ENCODED_BYTES_LEN;
        #[link_section = "__wasm_bindgen_unstable"]
        #[allow(long_running_const_eval)]
        static _GENERATED: [u8; _LEN] = flat_byte_slices([
            _PREFIX_JSON_BYTES,
            _ENCODED_BYTES,
        ]);
    };
}
pub use note::*;
