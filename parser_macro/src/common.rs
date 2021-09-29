use proc_macro2::Span;
use syn::Ident;

pub fn make_ident<S: AsRef<str>>(string: S) -> Ident {
    Ident::new(string.as_ref(), Span::call_site())
}