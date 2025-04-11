use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Ident, Path, Result, Type, token};
#[allow(unused)]
pub struct Argument {
    pub name: Ident,
    pub colon_token: token::Colon,
    pub ty: Type,
}

impl Parse for Argument {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Argument { name: input.parse()?, colon_token: input.parse()?, ty: input.parse()? })
    }
}

#[allow(unused)]
pub struct FunctionSpec {
    pub brace_token: token::Brace,
    pub name: Ident,
    pub comma1_token: token::Comma,
    pub paren_token: token::Paren,
    pub args: Punctuated<Argument, token::Comma>,
    pub comma2_token: Option<token::Comma>,
    pub arrow_token: token::RArrow,
    pub ret_type: Type,
}

impl Parse for FunctionSpec {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let brace_token = syn::braced!(content in input);
        let name = content.parse()?;
        let comma1_token = content.parse()?;
        let args_content;
        let paren_token = syn::parenthesized!(args_content in content);
        let args = Punctuated::parse_terminated(&args_content)?;
        let comma2_token = content.parse()?; // Try parsing comma
        let arrow_token = content.parse()?;
        let ret_type = content.parse()?;
        Ok(FunctionSpec {
            brace_token,
            name,
            comma1_token,
            paren_token,
            args,
            comma2_token,
            arrow_token,
            ret_type,
        })
    }
}

#[allow(unused)]
pub struct MacroInput {
    pub namespace: Path,
    pub comma_token: token::Comma,
    pub generic_type: Type,
    pub semicolon_token: token::Semi,
    pub functions: Punctuated<FunctionSpec, token::Comma>,
}
impl Parse for MacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(MacroInput {
            namespace: input.parse()?,
            comma_token: input.parse()?,
            generic_type: input.parse()?,
            semicolon_token: input.parse()?,
            functions: Punctuated::parse_terminated(input)?,
        })
    }
}
