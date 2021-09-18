use syn::FieldsNamed;
use proc_macro2::TokenStream;
use syn::Generics;
use syn::Ident;
use quote::quote;

pub fn get_field_names(fields: FieldsNamed) -> Vec<Ident> {
    fields.named.into_iter().map(|f| f.ident.unwrap()).collect()
}

pub fn impl_autoclone_named(ident: Ident, generics: Generics, field_names: FieldsNamed) -> TokenStream {
    let body = quote! {};

    for name in get_field_names(field_names) {
        body.extend(quote! {
            #name: #name.clone(),
        });
    }

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let quote = quote! {
        impl #impl_generics Clone for #ident #type_generics #where_clause {
            fn clone(&self) -> Self {
                Self {
                    #body
                }
            }
        }
    };

    quote.into()
}
