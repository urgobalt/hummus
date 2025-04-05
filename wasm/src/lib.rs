pub mod note;
#[cfg(not(target_arch = "wasm32"))]
compile_error!("This crate only works with wasm32-unknown-unknown");
