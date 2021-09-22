use proc_macro2::TokenStream;
use syn::GenericParam;
use syn::Generics;
use syn::Ident;
use syn::Type;
use syn::parse_quote;
use quote::quote;

pub fn bitor(ident: Ident, mut generics: Generics, toutput: Type) -> TokenStream {
    let totheroutput: GenericParam = parse_quote! {
        __TOtherOutput
    };

    let parserbound: GenericParam = parse_quote! {
        __TOtherParser: crate::parser::parser::Parser<Output = #totheroutput>
    };

    let gc = generics.clone();
    let (_, type_generics, _) = gc.split_for_impl();

    generics.params.push(totheroutput.clone());
    generics.params.push(parserbound.clone());
    let (impl_generics, _, where_clause) = generics.split_for_impl();

    let quote = quote! {
        impl #impl_generics std::ops::BitOr<__TOtherParser> for #ident #type_generics #where_clause {
            type Output = EitherParser<#toutput, Self, #totheroutput, __TOtherParser, Result<#toutput, #totheroutput>, fn(#toutput) -> Result<#toutput, #totheroutput>, fn(#totheroutput) -> Result<#toutput, #totheroutput>>;

            fn bitor(self, rhs: __TOtherParser) -> Self::Output {
                EitherParser::new(self, rhs)
            }
        }
    };

    quote.into()
}
