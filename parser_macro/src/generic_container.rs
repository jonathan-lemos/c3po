use syn::Ident;
use syn::Type;
use syn::WhereClause;
use syn::TypeGenerics;
use syn::ImplGenerics;
use syn::token::Add;
use syn::TypeParamBound;
use syn::punctuated::Punctuated;
use syn::TypeParam;
use syn::Generics;
use syn::parse_quote;

pub struct GenericContainer {
    original_generics: Generics,
    new_generics: Generics
}

impl GenericContainer {
    pub fn new(generics: Generics) -> Self {
        GenericContainer {
            original_generics: generics.clone(),
            new_generics: generics.clone()
        }
    }

    #[allow(dead_code)]
    pub fn push(&mut self, ident: Ident) -> Type {
        let param: TypeParam = parse_quote! {
            #ident
        };

        self.new_generics.params.push(syn::GenericParam::Type(param));
        parse_quote! {#ident}
    }

    pub fn push_bounded(&mut self, ident: Ident, bounds: Punctuated<TypeParamBound, Add>) -> Type {
        let param: TypeParam = parse_quote! {
            #ident: #bounds
        };

        self.new_generics.params.push(syn::GenericParam::Type(param));
        parse_quote! {#ident}
    }

    pub fn impl_generics(&self) -> ImplGenerics<'_> {
        self.new_generics.split_for_impl().0
    }

    pub fn type_generics(&self) -> TypeGenerics<'_> {
        self.original_generics.split_for_impl().1
    }

    pub fn where_clause(&self) -> Option<&WhereClause> {
        self.new_generics.split_for_impl().2
    }

    pub fn split_for_impl(&self) -> (ImplGenerics<'_>, TypeGenerics<'_>, Option<&WhereClause>) {
        (self.impl_generics(), self.type_generics(), self.where_clause())
    }
}