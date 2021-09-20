use syn::ExprClosure;
use syn::token::Comma;
use syn::Expr;
use syn::punctuated::Punctuated;
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

fn concat_tuple_type(tuple: TypeTuple, other: Type) -> (TypeTuple, ExprClosure) {
    let tuple_ident: Ident = parse_quote! {__a};
    let other_ident: Ident = parse_quote! {__b};

    let tuple_guts = enumerate_tuple_fields(tuple, tuple_ident);

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

pub fn add_normal(ident: Ident, generics: Generics, toutput: Type) -> TokenStream {
    let totheroutput: GenericParam = parse_quote! {
        __TOtherOutput
    };

    let parserbound: GenericParam = parse_quote! {
        __TOtherParser: Parser<Output = #totheroutput>
    };

    let tfinaloutput: Type = parse_quote! {
        (#toutput, #totheroutput)
    };

    generics.params.push(totheroutput);
    generics.params.push(parserbound);
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let quote = quote! {
        impl #impl_generics std::ops::Add<__TOtherParser> for #ident #type_generics #where_clause {
            type Output = ComposeParser<#toutput, Self, #totheroutput, #parserbound, #tfinaloutput, fn(#toutput, #totheroutput) -> #tfinaloutput>;

            fn add(self, rhs: __TOtherParser) -> Self::Output {
                ComposeParser::new(self, rhs);
            }
        }
    };

    quote.into()
}

pub fn add_tuple(ident: Ident, generics: Generics, toutput: TypeTuple) -> TokenStream {
    let totheroutput: GenericParam = parse_quote! {
        __TOtherOutput
    };

    let totheroutputtype: Type = parse_quote! {
        __TOtherOutput
    };

    let parserbound: GenericParam = parse_quote! {
        __TOtherParser: Parser<Output = #totheroutput>
    };

    let (tfinaloutput, combiner) = concat_tuple_type(toutput, totheroutputtype);

    generics.params.push(totheroutput);
    generics.params.push(parserbound);
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let quote = quote! {
        impl #impl_generics std::ops::Add<__TOtherParser> for #ident #type_generics #where_clause {
            type Output = ComposeParser<#toutput, Self, #totheroutput, #parserbound, #tfinaloutput, fn(#toutput, #totheroutput) -> #tfinaloutput>;

            fn add(self, rhs: __TOtherParser) -> Self::Output {
                ComposeParser::with_combiner(self, rhs, #combiner);
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
