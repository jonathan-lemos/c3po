use crate::common::push_new_parser;
use crate::common::ParserType;
use crate::generic_container::GenericContainer;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_quote;
use syn::Generics;
use syn::Ident;
use syn::Type;

pub fn impl_add(ident: Ident, generics: Generics, toutput: Type) -> TokenStream {
    let mut gc = GenericContainer::new(generics);
    let ParserType {
        inner: totheroutput,
        parser: totherparser,
    } = push_new_parser(&mut gc, "__TOtherOutput", "__TOtherParser");

    let tfinaloutput: Type = parse_quote! { (#toutput, #totheroutput) };

    let (impl_generics, type_generics, where_clause) = gc.split_for_impl();

    let quote = quote! {
        impl #impl_generics std::ops::Add<#totherparser> for #ident #type_generics #where_clause {
            type Output = crate::parsers::compose::composeparser::ComposeParser<#toutput, Self, #totheroutput, #totherparser, #tfinaloutput, fn(#toutput, #totheroutput) -> #tfinaloutput>;

            fn add(self, rhs: #totherparser) -> Self::Output {
                crate::parsers::compose::composeparser::ComposeParser::new(self, rhs)
            }
        }
    };

    quote.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::ItemImpl;

    #[test]
    fn add_compiles() {
        let output = impl_add(
            parse_quote! {Foo},
            parse_quote! {<T1, TS: Into<String>>},
            parse_quote! {i32},
        );
        let impl_block = syn::parse2::<ItemImpl>(output);

        assert!(impl_block.is_ok());
    }
}
