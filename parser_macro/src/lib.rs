mod clone;
mod mul;

use proc_macro::TokenStream;
use quote::quote;
use syn::{FieldsNamed, Generics, Ident, ItemStruct};


#[proc_macro_attribute]
pub fn parser(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_clone = item.clone();
    let input = syn::parse_macro_input!(item_clone as ItemStruct);

    let ItemInfo {ident, generics} = get_info(input).expect("#[parser] can only be used on an enum or a struct");
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let output = quote! {
        #item

        impl #impl_generics Mul<usize> for #ident #type_generics #where_clause {
            type Output = RepeatParser<Self>

            fn mul
        }
    };

    output.into()
}