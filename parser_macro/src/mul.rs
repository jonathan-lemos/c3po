use proc_macro2::TokenStream;
use quote::quote;
use syn::{Generics, Ident};

pub fn impl_mul(ident: Ident, generics: Generics) -> TokenStream {
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let quote = quote! {
        impl #impl_generics Mul<usize> for #ident #type_generics #where_clause {
            type Output = RepeatParser<Self>

            fn mul(self, rhs: Self) -> Self::Output {
                Self {
                }
            }
        }
    };

    quote.into()
}
