use crate::generic_container::GenericContainer;
use proc_macro2::TokenStream;
use syn::Generics;
use syn::Ident;
use syn::Type;
use syn::parse_quote;
use quote::quote;

pub fn bitor(ident: Ident, generics: Generics, toutput: Type) -> TokenStream {
    let mut gc = GenericContainer::new(generics);
    let totheroutput = gc.push_bounded("__TOtherOutput", parse_quote!{Send + Sync});
    let totherparser = gc.push_bounded("__TOtherParser", parse_quote!{crate::parser::parser::Parser<Output = #totheroutput>});

    let (impl_generics, type_generics, where_clause) = gc.split_for_impl();

    let quote = quote! {
        impl #impl_generics std::ops::BitOr<#totherparser> for #ident #type_generics #where_clause {
            type Output = crate::parsers::either::eitherparser::EitherParser<#toutput, Self, #totheroutput, #totherparser, Result<#toutput, #totheroutput>, fn(#toutput) -> Result<#toutput, #totheroutput>, fn(#totheroutput) -> Result<#toutput, #totheroutput>>;

            fn bitor(self, rhs: __TOtherParser) -> Self::Output {
                crate::parsers::either::eitherparser::EitherParser::new(self, rhs)
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
    fn bitor_compiles() {
        let output = bitor(parse_quote!{Foo}, parse_quote!{<T1, TS: Into<String>>}, parse_quote!{i32});
        let impl_block = syn::parse2::<ItemImpl>(output);

        assert!(impl_block.is_ok());
    }
}
