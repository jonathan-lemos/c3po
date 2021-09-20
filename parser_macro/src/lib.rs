mod add;
mod bitor;
mod clone;
mod mul;

use proc_macro::TokenStream;
use syn::{ItemStruct, Type};


#[proc_macro_attribute]
pub fn parser(attr: TokenStream, item: TokenStream) -> TokenStream {
    let parser_type = syn::parse_macro_input!(attr as Type);

    let item_clone = item.clone();
    let input = syn::parse_macro_input!(item_clone as ItemStruct);

    item.extend::<TokenStream>(add::add(input.ident, input.generics, parser_type).into());
    item.extend::<TokenStream>(bitor::bitor(input.ident, input.generics, parser_type).into());
    item.extend::<TokenStream>(clone::impl_autoclone(input.ident, input.generics, input).into());
    item.extend::<TokenStream>(mul::impl_mul(input.ident, input.generics, parser_type).into());

    item
}