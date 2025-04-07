// generator.rs
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::spanned::Spanned;
use syn::{Result, Type, TypeReference, parse_quote}; // Use syn 2.0 Result

use super::parser::{Argument, MacroInput};

/// Checks if a syn::Type represents a reference (e.g., &T, &'a T, &'a mut T)
fn is_reference_type(ty: &Type) -> bool {
    matches!(ty, Type::Reference(_))
}

/// Gets owned type
fn get_wasm_owned_type(original_type: &Type) -> Result<Type> {
    match original_type {
        Type::Reference(TypeReference { elem, .. }) => {
            let inner_type_str = elem.to_token_stream().to_string().replace(' ', "");
            match inner_type_str.as_str() {
                "str" => Ok(parse_quote!(String)),
                "[u8]" => Ok(parse_quote!(Vec<u8>)),
                "Path" => Ok(parse_quote!(std::path::PathBuf)),
                _ => get_wasm_owned_type(elem),
            }
        }
        _ => {
            // Not a reference type, assume it's already the desired owned type.
            Ok(original_type.clone())
        }
    }
}

pub fn generate_code(input: MacroInput) -> Result<TokenStream2> {
    let namespace = &input.namespace;
    let generic_type = &input.generic_type;
    let mut generated_functions = Vec::new();

    for func_spec in &input.functions {
        let func_name = &func_spec.name;
        let ret_type = &func_spec.ret_type;

        let mut wasm_signature_args = Vec::new();
        for arg in &func_spec.args {
            let name = &arg.name;
            let wasm_owned_type = get_wasm_owned_type(&arg.ty).map_err(|e| {
                syn::Error::new(
                    arg.ty.span(),
                    format!(
                        "Failed to determine owned type for '{}': {}",
                        arg.ty.to_token_stream(),
                        e
                    ),
                )
            })?; // Add error context
            wasm_signature_args.push(quote! { #name : #wasm_owned_type });
        }

        let mut call_args = Vec::new();
        for arg in &func_spec.args {
            let name = &arg.name;
            if is_reference_type(&arg.ty) {
                call_args.push(quote! { &#name });
            } else {
                call_args.push(quote! { #name });
            }
        }

        let func_code = quote! {
            #[wasm_bindgen::prelude::wasm_bindgen]
            pub async fn #func_name ( #( #wasm_signature_args ),* ) -> #ret_type {
                ( #namespace :: #func_name :: <#generic_type> ( #( #call_args ),* ) ).await
            }
        };
        generated_functions.push(func_code);
    }

    // Combine all generated functions
    Ok(quote! {
        #( #generated_functions )*
    })
}
