use proc_macro2::Span;
use proc_macro2::TokenStream;
use crate::generic_container::GenericContainer;
use syn::Expr;
use syn::punctuated::Punctuated;
use syn::Type;
use syn::ExprClosure;
use syn::TypeTuple;
use syn::parse_quote;
use syn::ItemImpl;
use syn::token::Comma;
use syn::LitInt;
use quote::ToTokens;

fn append_to_tuple(tuple: TypeTuple, other: Type) -> (TypeTuple, ExprClosure) {
    let mut guts = tuple.elems.clone();
    guts.push(other);

    let new_type: TypeTuple = parse_quote! {(#guts)};

    let old_guts: Punctuated<Expr, Comma> = (0..tuple.elems.len()).map(|i| {
        let num = LitInt::new(&i.to_string(), Span::call_site());
        let expr: Expr = parse_quote!{a.#num};
        expr
    }).collect();

    let closure: ExprClosure = parse_quote! {
        |a, b| {
            (#old_guts, b)
        }
    };

    (new_type, closure)
}

fn compose_parser_for(toutput: Type, tleftparser: Type, trightoutput: Type, trightparser: Type, tfinaloutput: TypeTuple) -> Type {
    parse_quote! {
        crate::parsers::compose::composeparser::ComposeParser<#toutput, #tleftparser, #trightoutput, #trightparser, #tfinaloutput, fn(#toutput, #trightoutput) -> #tfinaloutput>
    }
}

fn impl_bitand_for(impl_for: Type, totherparser: Type, output_parser: Type, generics: &mut GenericContainer, closure: ExprClosure) -> ItemImpl {
    let (impl_generics, _, where_clause) = generics.split_for_impl();

    parse_quote! {
        impl #impl_generics std::ops::BitAnd<#totherparser> for #impl_for #where_clause {
            type Output = #output_parser;

            fn bitand(self, rhs: #totherparser) -> Self::Output {
                crate::parsers::compose::composeparser::ComposeParser::using_combiner(self, rhs, #closure)
            }
        }
    }
}

fn make_type(n: usize) -> String {
    format!("__T{}", n)
}

pub fn impl_bitand() -> TokenStream {
    let mut ts = TokenStream::new();

    let mut gc = GenericContainer::new(parse_quote! {});
    let t0 = gc.push_bounded(make_type(0), parse_quote! {Send + Sync});
    let tleftparser = gc.push_bounded("__TLeftParser", parse_quote!{crate::parser::parser::Parser<Output = #t0>});

    let t1 = gc.push_bounded(make_type(1), parse_quote! {Send + Sync});
    let trightparser = gc.push_bounded("__TRightParser", parse_quote!{crate::parser::parser::Parser<Output = #t1>});

    let mut tuple_type: TypeTuple = parse_quote!{(#t0, #t1)};
    let mut compose_parser_type = compose_parser_for(t0.clone(), tleftparser.clone(), t1.clone(), trightparser.clone(), tuple_type.clone());

    for i in 2..32 {
        let tnext = gc.push_bounded(make_type(i), parse_quote! {Send + Sync});
        let tnextparser = gc.push_bounded("__TNextParser", parse_quote!{crate::parser::parser::Parser<Output = #tnext>});

        let (tuple_type_next, appender) = append_to_tuple(tuple_type.clone(), tnext.clone());
        // this needs to have a parser returning tnext
        let new_compose_parser = compose_parser_for(syn::Type::Tuple(tuple_type.clone()), parse_quote! {Self}, tnext.clone(), tnextparser.clone(), tuple_type_next.clone());

        let impl_block = impl_bitand_for(compose_parser_type.clone(), tnextparser.clone(), new_compose_parser.clone(), &mut gc, appender);
        impl_block.to_tokens(&mut ts);

        let tl = gc.push_bounded("__TLeftParser", parse_quote!{crate::parser::parser::Parser<Output = #tuple_type>});
        let tr = gc.push_bounded("__TRightParser", parse_quote!{crate::parser::parser::Parser<Output = #tnext>});
        compose_parser_type = compose_parser_for(syn::Type::Tuple(tuple_type.clone()), tl.clone(), tnext.clone(), tr.clone(), tuple_type_next.clone());
        tuple_type = tuple_type_next;
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

        let file = syn::parse2::<File>(tokens);
        assert!(file.is_ok());
    }
}
