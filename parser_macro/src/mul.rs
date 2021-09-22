use proc_macro2::TokenStream;
use quote::quote;
use syn::{Generics, GenericParam, Ident, parse_quote, Type};

pub fn impl_mul_usize(ident: Ident, generics: Generics, toutput: Type) -> TokenStream {
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let quote = quote! {
        impl #impl_generics std::ops::Mul<usize> for #ident #type_generics #where_clause {
            type Output = crate::parsers::repeat::repeatparser::RepeatParser<#toutput, Self>;

            fn mul(self, rhs: usize) -> Self::Output {
                c3po::parsers::repeat::repeatparser::RepeatParser::count(self, rhs)
            }
        }
    };

    quote.into()
}

pub fn impl_mul_range(ident: Ident, mut generics: Generics, toutput: Type) -> TokenStream {
    let rangebound: GenericParam = parse_quote! {
        __TRangeBound: std::ops::RangeBounds<usize>
    };

    let gc = generics.clone();
    let (_, type_generics, _) = gc.split_for_impl();

    generics.params.push(rangebound);
    let (impl_generics, _, where_clause) = generics.split_for_impl();

    let quote = quote! {
        impl #impl_generics std::ops::Mul<__TRangeBound> for #ident #type_generics #where_clause {
            type Output = crate::parsers::repeat::repeatparser::RepeatParser<#toutput, Self>;

            fn mul(self, rhs: __TRangeBound) -> Self::Output {
                crate::parsers::repeat::repeatparser::RepeatParser::range(self, rhs)
            }
        }
    };

    quote.into()
}

pub fn impl_mul(ident: Ident, generics: Generics, toutput: Type) -> TokenStream {
    let mut t1 = impl_mul_usize(ident.clone(), generics.clone(), toutput.clone());
    t1.extend(impl_mul_range(ident.clone(), generics.clone(), toutput.clone()));
    t1
}
