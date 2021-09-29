use crate::generic_container::GenericContainer;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Generics, Ident, parse_quote, Type};

#[allow(dead_code)]
// can't use this because downstream crates can implement RangeBounds for usize
pub fn impl_mul_usize(ident: Ident, generics: Generics, toutput: Type) -> TokenStream {
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let quote = quote! {
        impl #impl_generics std::ops::Mul<usize> for #ident #type_generics #where_clause {
            type Output = crate::parsers::repeat::repeatparser::RepeatParser<#toutput, Self>;

            fn mul(self, rhs: usize) -> Self::Output {
                crate::parsers::repeat::repeatparser::RepeatParser::count(self, rhs)
            }
        }
    };

    quote.into()
}

pub fn impl_mul_range(ident: Ident, generics: Generics, toutput: Type) -> TokenStream {
    let mut gc = GenericContainer::new(generics);

    gc.push_bounded("__TRangeBound", parse_quote! {std::ops::RangeBounds<usize>});

    let (impl_generics, type_generics, where_clause) = gc.split_for_impl();

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
    impl_mul_range(ident.clone(), generics.clone(), toutput.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::File;

    #[test]
    fn mul_compiles() {
        let output = impl_mul(parse_quote!{Foo}, parse_quote!{<T1, TS: Into<String>>}, parse_quote!{i32});
        let impl_block = syn::parse2::<File>(output);

        assert!(impl_block.is_ok());
    }
}
