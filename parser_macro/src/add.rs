use crate::generic_container::GenericContainer;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::ExprClosure;
use syn::Expr;
use syn::TypeTuple;
use proc_macro2::TokenStream;
use syn::GenericParam;
use syn::Generics;
use syn::Ident;
use syn::Type;
use syn::parse_quote;
use quote::quote;

fn enumerate_tuple_fields(tuple: TypeTuple, ident: Ident) -> impl Iterator<Item = Expr> {
    (0..tuple.elems.len()).map(move |i| {
        let selector: Expr = parse_quote! {
            #ident.#i
        };
        selector
    })
}

fn concat_tuple_type(mut tuple: TypeTuple, other: Type) -> (TypeTuple, ExprClosure) {
    let tuple_ident: Ident = parse_quote! {__a};
    let other_ident: Ident = parse_quote! {__b};

    let tuple_guts: Punctuated<Expr, Comma> = enumerate_tuple_fields(tuple.clone(), tuple_ident.clone()).collect();

    tuple.elems.push(other);
    let typ = TypeTuple {
        paren_token: tuple.paren_token,
        elems: tuple.elems
    };

    let closure: ExprClosure = parse_quote! {
        |#tuple_ident, #other_ident| {
            (#tuple_guts, #other_ident)
        }
    };

    (typ, closure)
}

fn add_normal(ident: Ident, generics: Generics, toutput: Type) -> TokenStream {
    let mut gc = GenericContainer::new(generics);
    let totheroutput = gc.push(parse_quote! {__TOtherOutput}, parse_quote!{Send + Sync});
    let totherparser = gc.push(parse_quote! {__TOtherParser}, parse_quote!{crate::parser::parser::Parser<Output = #totheroutput>});

    let tfinaloutput: Type = parse_quote! {
        (#toutput, #totheroutput)
    };

    let (impl_generics, type_generics, where_clause) = gc.split_for_impl();

    let quote = quote! {
        impl #impl_generics std::ops::Add<__TOtherParser> for #ident #type_generics #where_clause {
            type Output = crate::parsers::compose::composeparser::ComposeParser<#toutput, Self, #totheroutput, #totherparser, #tfinaloutput, fn(#toutput, #totheroutput) -> #tfinaloutput>;

            fn add(self, rhs: __TOtherParser) -> Self::Output {
                crate::parsers::compose::composeparser::ComposeParser::new(self, rhs)
            }
        }
    };

    quote.into()
}

fn add_tuple(ident: Ident, mut generics: Generics, toutput: TypeTuple) -> TokenStream {
    let totheroutput: GenericParam = parse_quote! {
        __TOtherOutput: Send + Sync
    };

    let totheroutputtype: Type = parse_quote! {
        __TOtherOutput
    };

    let parsertype: GenericParam = parse_quote! {
        __TOtherParser
    };

    let parserbound: GenericParam = parse_quote! {
        __TOtherParser: crate::parser::parser::Parser<Output = #totheroutput>
    };

    let (tfinaloutput, combiner) = concat_tuple_type(toutput.clone(), totheroutputtype.clone());

    let gc = generics.clone();
    let (_, type_generics, _) = gc.split_for_impl();

    generics.params.push(totheroutput.clone());
    generics.params.push(parserbound.clone());
    let (impl_generics, _, where_clause) = generics.split_for_impl();

    let quote = quote! {
        impl #impl_generics std::ops::Add<__TOtherParser> for #ident #type_generics #where_clause {
            type Output = crate::parsers::compose::composeparser::ComposeParser<#toutput, Self, #totheroutput, #parsertype, #tfinaloutput, fn(#toutput, #totheroutput) -> #tfinaloutput>;

            fn add(self, rhs: __TOtherParser) -> Self::Output {
                crate::parsers::compose::composeparser::ComposeParser::with_combiner(self, rhs, #combiner)
            }
        }
    };

    quote.into()
}

pub fn add(ident: Ident, generics: Generics, toutput: Type) -> TokenStream {
    match toutput {
        syn::Type::Tuple(tt) => add_tuple(ident, generics, tt),
        _ => add_normal(ident, generics, toutput)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use syn::File;

    #[test]
    fn add_compiles() {
        let output = add(parse_quote!{Foo}, parse_quote!{<T1, TS: Into<String>>}, parse_quote!{i32});
        let file = syn::parse2::<File>(output);

        assert!(file.is_ok());
    }
}