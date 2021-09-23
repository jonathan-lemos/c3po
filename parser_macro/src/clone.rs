use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::ItemStruct;
use syn::Expr;
use syn::FieldsUnnamed;
use syn::FieldsNamed;
use proc_macro2::TokenStream;
use syn::Generics;
use syn::Ident;
use quote::quote;
use syn::parse_quote;

fn get_field_names(fields: FieldsNamed) -> Vec<Ident> {
    fields.named.into_iter().map(|f| f.ident.unwrap()).collect()
}

fn impl_autoclone_named(ident: Ident, generics: Generics, field_names: FieldsNamed) -> TokenStream {
    let mut body = quote! {};

    for name in get_field_names(field_names) {
        body.extend(quote! {
            #name: self.#name.clone(),
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

fn tuple_guts_clone(fields: FieldsUnnamed) -> impl Iterator<Item = Expr> {
    (0..fields.unnamed.len()).map(move |i| {
        let e: Expr = parse_quote! {
            self.#i.clone()
        };
        e
    })
}

fn impl_autoclone_unnamed(ident: Ident, generics: Generics, fields: FieldsUnnamed) -> TokenStream {
    let guts: Punctuated<Expr, Comma> = tuple_guts_clone(fields).collect();
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let quote = quote! {
        impl #impl_generics Clone for #ident #type_generics #where_clause {
            fn clone(&self) -> Self {
                Self(#guts)
            }
        }        
    };

    quote.into()
}

fn impl_autoclone_unit(ident: Ident, generics: Generics) -> TokenStream {
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let quote = quote! {
        impl #impl_generics Clone for #ident #type_generics #where_clause {
            fn clone(&self) -> Self {
                Self
            }
        }        
    };

    quote.into()
}

pub fn impl_autoclone(type_struct: ItemStruct) -> TokenStream {
    let ident = type_struct.ident;
    let generics = type_struct.generics;

    match type_struct.fields {
        syn::Fields::Named(n) => impl_autoclone_named(ident, generics, n),
        syn::Fields::Unnamed(u) => impl_autoclone_unnamed(ident, generics, u),
        syn::Fields::Unit => impl_autoclone_unit(ident, generics)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::ItemImpl;

    #[test]
    fn clone_compiles_for_named_struct() {
        let output = impl_autoclone(parse_quote!{
            struct Foo<T1, TS: Into<String>> {
                thing: Vec<T1>,
                thang: TS
            }
        });
        let impl_block = syn::parse2::<ItemImpl>(output);

        assert!(impl_block.is_ok());
    }
}
