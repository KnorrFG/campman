use base64::{engine::general_purpose, Engine as _};
use litrs::Literal;
use proc_macro::{Literal as PLiteral, TokenStream, TokenTree};

use std::fs;

/// Loads the file, and returns a base64 encoded String literal of its contents
#[proc_macro]
pub fn b64_embed(arg: TokenStream) -> TokenStream {
    let mut arg = arg.into_iter().collect::<Vec<_>>();
    assert!(arg.len() == 1, "Only a single arg allowed");

    if let Ok(Literal::String(s)) = Literal::try_from(arg.pop().unwrap()) {
        let s = s.value();
        let bytes = fs::read(s).expect(&format!("Loading file: {}", s));
        let b64 = general_purpose::STANDARD.encode(&bytes);
        TokenTree::Literal(PLiteral::string(&b64)).into()
    } else {
        panic!("The argument must be a string literal");
    }
}
