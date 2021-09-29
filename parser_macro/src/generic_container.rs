use crate::common::make_ident;
use syn::token::Comma;
use syn::GenericParam;
use std::collections::HashMap;
use syn::Type;
use syn::WhereClause;
use syn::TypeGenerics;
use syn::ImplGenerics;
use syn::token::Add;
use syn::TypeParamBound;
use syn::punctuated::Punctuated;
use syn::Generics;
use syn::parse_quote;
use quote::quote;

#[derive(Clone)]
pub struct GenericContainer {
    original_generics: Generics,
    new_other_generics: Punctuated<GenericParam, Comma>,
    new_type_generics: HashMap<String, Punctuated<TypeParamBound, Add>>,
    split_buffer: Generics
}

impl GenericContainer {
    pub fn new(generics: Generics) -> Self {
        let orig = generics.clone();

        let other = generics.clone().params.into_iter().filter(|x| match x {
            syn::GenericParam::Type(_) => false,
            _ => true
        }).collect();

        let new = generics.clone().params.into_iter().filter_map(|x| match x {
            syn::GenericParam::Type(t) => {
                Some((t.ident.to_string(), t.bounds))
            },
            _ => None
        }).collect();

        GenericContainer {
            original_generics: generics.clone(),
            new_other_generics: other,
            new_type_generics: new,
            split_buffer: generics.clone()
        }
    }

    #[allow(dead_code)]
    pub fn push<S: Into<String>>(&mut self, ident: S) -> Type {
        self.push_bounded(ident, Punctuated::new())
    }

    pub fn push_bounded<S: Into<String>>(&mut self, ident: S, bounds: Punctuated<TypeParamBound, Add>) -> Type {
        let string = ident.into();

        self.new_type_generics.insert(string.clone(), bounds);
        syn::parse2(string.parse().unwrap()).unwrap()
    }

    pub fn remove<S: Into<String>>(&mut self, ident: S) -> bool {
        self.new_type_generics.remove(&ident.into()).is_some()
    }

    fn new_generics(&self) -> Generics {
        let mut tmp = self.new_other_generics.clone();

        let t = self.new_type_generics.iter().map(|(a, b)| {
            let ident = make_ident(a);

            let output: GenericParam = if b.len() > 0 {
                parse_quote! {#ident: #b}
            } else {
                parse_quote! {#ident}
            };
            output
        });

        tmp.extend(t);
        parse_quote! {<#tmp>}
    }

    pub fn split_for_impl(&mut self) -> (ImplGenerics<'_>, TypeGenerics<'_>, Option<&WhereClause>) {
        self.split_buffer = self.new_generics();
        let (i, _, w) = self.split_buffer.split_for_impl();
        return (i, self.type_generics(), w);
    }

    pub fn type_generics(&self) -> TypeGenerics<'_> {
        self.original_generics.split_for_impl().1
    }
}