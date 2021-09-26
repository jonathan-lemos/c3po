use proc_macro2::Span;
use proc_macro2::TokenStream;
use syn::Ident;
use crate::generic_container::GenericContainer;
use syn::Expr;
use syn::punctuated::Punctuated;
use syn::Type;
use syn::ExprClosure;
use syn::TypeTuple;
use syn::parse_quote;
use syn::ItemImpl;
use syn::token::Comma;
use quote::ToTokens;

fn append_to_tuple(tuple: TypeTuple, other: Type) -> (TypeTuple, ExprClosure) {
    let mut guts = tuple.elems.clone();
    guts.push(other);

    let new_type: TypeTuple = parse_quote! {(#guts)};

    let old_guts: Punctuated<Expr, Comma> = tuple.elems.into_iter().map(|e| {
        let expr: Expr = parse_quote!{a.#e};
        expr
    }).collect();

    let closure: ExprClosure = parse_quote! {
        |a, b| {
            (#old_guts, b)
        }
    };

    (new_type, closure)
}

fn compose_parser_for(toutput: Type, totheroutput: Type, totherparser: Type, tfinaloutput: TypeTuple) -> Type {
    parse_quote! {
        crate::parsers::compose::composeparser::ComposeParser<#toutput, Self, #totheroutput, #totherparser, #tfinaloutput, fn(#toutput, #totheroutput) -> #tfinaloutput>
    }
}

fn impl_bitor_for(impl_for: Type, totherparser: Type, output_parser: Type, generics: &GenericContainer, closure: ExprClosure) -> ItemImpl {
    let (impl_generics, _, where_clause) = generics.split_for_impl();

    parse_quote! {
        impl #impl_generics std::ops::BitAnd<#totherparser> for #impl_for #where_clause {
            type Output = #output_parser;

            fn bitand(self, rhs: #totherparser) -> Self::Output {
                crate::parsers::compose::composeparser::ComposeParser::with_combiner(self, rhs, #closure)
            }
        }
    }
}

fn make_type(n: usize) -> Ident {
    let s = format!("__T{}", n);
    Ident::new(&s, Span::call_site())
}

pub fn impl_bitand() -> TokenStream {
    let mut ts = TokenStream::new();

    let mut gc = GenericContainer::new(parse_quote! {});
    let totheroutput = gc.push_bounded(parse_quote! {__TOtherOutput}, parse_quote! {Send + Sync});
    let totherparser = gc.push_bounded(parse_quote! {__TOtherParser}, parse_quote!{crate::parser::parser::Parser<Output = #totheroutput>});

    let t0 = gc.push_bounded(make_type(0), parse_quote! {Send + Sync});

    let mut tuple_type: TypeTuple = parse_quote!{(#t0, #totheroutput)};
    let mut compose_parser_type = compose_parser_for(t0.clone(), totheroutput.clone(), totherparser.clone(), tuple_type.clone());

    for i in 1..32 {
        let tnext = gc.push_bounded(make_type(i), parse_quote! {Send + Sync});
        let (tuple_type_next, appender) = append_to_tuple(tuple_type.clone(), tnext.clone());
        let new_compose_parser = compose_parser_for(syn::Type::Tuple(tuple_type.clone()), totheroutput.clone(), totherparser.clone(), tuple_type_next.clone());

        let impl_block = impl_bitor_for(compose_parser_type.clone(), totherparser.clone(), new_compose_parser.clone(), &gc, appender);
        impl_block.to_tokens(&mut ts);

        tuple_type = tuple_type_next;
        compose_parser_type = new_compose_parser;
    }

    ts
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::File;

    #[test]
    pub fn bitand_compiles() {
        let tokens = impl_bitand();
        println!("{}", tokens.clone().to_string());

        let file = syn::parse2::<File>(tokens);
        assert!(file.is_ok());
    }
}
