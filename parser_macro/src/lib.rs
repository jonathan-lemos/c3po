mod add;
mod bitand;
mod bitor;
mod clone;
mod generic_container;
mod mul;

use proc_macro::TokenStream;
use syn::{ItemStruct, Type};


#[proc_macro_attribute]
pub fn parser(attr: TokenStream, mut item: TokenStream) -> TokenStream {
    let parser_type = syn::parse_macro_input!(attr as Type);

    let item_clone = item.clone();
    let input = syn::parse_macro_input!(item_clone as ItemStruct);

    item.extend::<TokenStream>(add::impl_add(input.ident.clone(), input.generics.clone(), parser_type.clone()).into());
    item.extend::<TokenStream>(bitor::bitor(input.ident.clone(), input.generics.clone(), parser_type.clone()).into());
    item.extend::<TokenStream>(clone::impl_autoclone(input.clone()).into());
    item.extend::<TokenStream>(mul::impl_mul(input.ident.clone(), input.generics.clone(), parser_type.clone()).into());

    item
}