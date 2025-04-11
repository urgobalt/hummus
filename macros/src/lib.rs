mod parser;
mod generator;
use proc_macro::TokenStream;
use syn::parse_macro_input;
#[proc_macro]
pub fn specialize_function_for_bindgen(input: TokenStream) -> TokenStream {
    let parsed_input = parse_macro_input!(input as parser::MacroInput);
    match generator::generate_code(parsed_input) {
        Ok(token_stream) => token_stream.into(),
        Err(err) => err.to_compile_error().into(), // Convert syn::Error to compiler error
    }
}
