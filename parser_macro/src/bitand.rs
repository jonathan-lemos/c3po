use crate::common::push_new_parser;
use crate::common::push_parser;
use crate::common::ParserType;
use crate::generic_container::GenericContainer;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse_quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::Expr;
use syn::ExprClosure;
use syn::ItemImpl;
use syn::LitInt;
use syn::Type;
use syn::TypeTuple;

fn append_to_tuple(tuple: &TypeTuple, other: &Type) -> (TypeTuple, ExprClosure) {
    let mut guts = tuple.elems.clone();
    guts.push(other.clone());

    let new_type: TypeTuple = parse_quote! {(#guts)};

    let old_guts: Punctuated<Expr, Comma> = (0..tuple.elems.len())
        .map(|i| {
            let num = LitInt::new(&i.to_string(), Span::call_site());
            let expr: Expr = parse_quote! {a.#num};
            expr
        })
        .collect();

    let closure: ExprClosure = parse_quote! {
        |a, b| {
            (#old_guts, b)
        }
    };

    (new_type, closure)
}

fn compose_parser_for(
    leftparser: &ParserType,
    rightparser: &ParserType,
    tfinaloutput: &TypeTuple,
) -> Type {
    let ParserType {
        inner: tlo,
        parser: tlp,
    } = leftparser;
    let ParserType {
        inner: tro,
        parser: trp,
    } = rightparser;

    parse_quote! {
        crate::parsers::compose::composeparser::ComposeParser<#tlo, #tlp, #tro, #trp, #tfinaloutput, fn(#tlo, #tro) -> #tfinaloutput>
    }
}

fn impl_bitand_for(
    impl_for: &Type,
    totherparser: &Type,
    output_parser: &Type,
    generics: &mut GenericContainer,
    closure: &ExprClosure,
) -> ItemImpl {
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

struct BitandImplState<'a, 'b> {
    token_stream: &'b mut TokenStream,
    gc: &'a mut GenericContainer,
    impl_for: Type,
    bitand_output: TypeTuple,
}

fn impl_bitand_iteration<'a, 'b>(state: BitandImplState<'a, 'b>, n: usize) -> BitandImplState<'a, 'b> {
    let BitandImplState {
        token_stream,
        gc,
        impl_for,
        bitand_output
    } = state;

    let nextparser = push_new_parser(gc, make_type(n), "__TNextParser");

    let tfinalparser = ParserType {
        inner: syn::Type::Tuple(bitand_output.clone()),
        parser: parse_quote! {Self},
    };

    let (tuple_type_next, appender) = append_to_tuple(&bitand_output, &nextparser.inner);
    let new_compose_parser = compose_parser_for(&tfinalparser, &nextparser, &tuple_type_next);

    let impl_block = impl_bitand_for(
        &impl_for,
        &nextparser.parser,
        &new_compose_parser,
        gc,
        &appender,
    );
    impl_block.to_tokens(token_stream);

    let tl = push_parser(gc, &bitand_output, "__TLeftParser");
    let tr = push_parser(gc, &nextparser.inner, "__TRightParser");

    BitandImplState {
        token_stream,
        gc,
        impl_for: compose_parser_for(&tl, &tr, &tuple_type_next),
        bitand_output: tuple_type_next
    }
}

pub fn impl_bitand() -> TokenStream {
    let mut ts = TokenStream::new();
    let mut gc = GenericContainer::new(parse_quote! {});

    let leftparser = push_new_parser(&mut gc, make_type(0), "__TLeftParser");
    let rightparser = push_new_parser(&mut gc, make_type(1), "__TRightParser");

    let (t0, t1) = (&leftparser.inner, &rightparser.inner);
    let bitand_output: TypeTuple = parse_quote! {(#t0, #t1)};

    let impl_for = compose_parser_for(&leftparser, &rightparser, &bitand_output);

    let initial_state = BitandImplState {
        token_stream: &mut ts,
        gc: &mut gc,
        bitand_output,
        impl_for
    };

    (2..32).fold(initial_state, impl_bitand_iteration);   

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
