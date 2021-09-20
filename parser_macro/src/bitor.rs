use proc_macro2::TokenStream;
use syn::GenericParam;
use syn::Generics;
use syn::Ident;
use syn::Type;
use syn::parse_quote;
use quote::quote;

pub fn bitor(ident: Ident, generics: Generics, toutput: Type) -> TokenStream {
    let totheroutput: GenericParam = parse_quote! {
        __TOtherOutput
    };

    let parserbound: GenericParam = parse_quote! {
        __TOtherParser: Parser<Output = #totheroutput>
    };

    generics.params.push(totheroutput);
    generics.params.push(parserbound);
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

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
