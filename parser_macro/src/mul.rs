use proc_macro2::TokenStream;
use quote::quote;
use syn::{Generics, GenericParam, Ident, parse_quote, Type};

pub fn impl_mul_usize(ident: Ident, generics: Generics, toutput: Type) -> TokenStream {
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let quote = quote! {
        impl #impl_generics std::ops::Mul<usize> for #ident #type_generics #where_clause {
            type Output = RepeatParser<#toutput, Self>

            fn mul(self, rhs: usize) -> Self::Output {
                crate::RepeatParser::count(self, rhs)
            }
        }
    };

    quote.into()
}

pub fn impl_mul_range(ident: Ident, generics: Generics, toutput: Type) -> TokenStream {
    let rangebound: GenericParam = parse_quote! {
        __TRangeBound: RangeBounds<usize>
    };

    generics.params.push(rangebound);
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let quote = quote! {
        impl #impl_generics std::ops::Mul<__TRangeBound> for #ident #type_generics #where_clause {
            type Output = RepeatParser<#toutput, Self>

            fn mul(self, rhs: __TRangeBound) -> Self::Output {
                crate::parsers::repeatparser::RepeatParser::range(self, rhs)
            }
        }
    };

    quote.into()
}

pub fn impl_mul(ident: Ident, generics: Generics, toutput: Type) -> TokenStream {
    let t1 = impl_mul_usize(ident, generics, toutput);
    t1.extend(impl_mul_range(ident, generics, toutput));
    t1
}
