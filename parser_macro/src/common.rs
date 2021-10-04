use crate::generic_container::GenericContainer;
use syn::Type;
use proc_macro2::TokenStream;
use quote::ToTokens;
use proc_macro2::Span;
use syn::Ident;
use syn::parse_quote;

pub fn make_ident<S: AsRef<str>>(string: S) -> Ident {
    Ident::new(string.as_ref(), Span::call_site())
}

pub fn push_parser<T: ToTokens, S: Into<String>>(gc: &mut GenericContainer, inner: T, parser_ident: S) -> ParserType {
    ParserType {
        inner: parse_quote! {#inner},
        parser: gc.push_bounded(parser_ident, parse_quote! {crate::parser::parser::Parser<Output = #inner>})
    }
}

pub struct ParserType {
    pub inner: Type,
    pub parser: Type
}

pub fn push_new_parser<S1: Into<String>, S2: Into<String>>(gc: &mut GenericContainer, inner_ident: S1, parser_ident: S2) -> ParserType {
    let t = gc.push_send_sync(inner_ident);
    push_parser(gc, &t, parser_ident)
}

#[allow(dead_code)]
pub fn debug_print<T: ToTokens>(t: &T) {
    let mut ts = TokenStream::new();
    t.to_tokens(&mut ts);
    println!("{}", ts.to_string());
}
