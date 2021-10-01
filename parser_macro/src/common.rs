use proc_macro2::TokenStream;
use quote::ToTokens;
use proc_macro2::Span;
use syn::Ident;

pub fn make_ident<S: AsRef<str>>(string: S) -> Ident {
    Ident::new(string.as_ref(), Span::call_site())
}

#[allow(dead_code)]
pub fn debug_print<T: ToTokens>(t: &T) {
    let mut ts = TokenStream::new();
    t.to_tokens(&mut ts);
    println!("{}", ts.to_string());
}