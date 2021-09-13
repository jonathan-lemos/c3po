use std::ops::RangeBounds;
use super::super::node::node::Node;
use crate::parser::parser::Parser;
use crate::parsers::map::map::Map;
use crate::parsers::repeat::repeatparser::RepeatParser;

pub fn repeat<TParser, TRange, TKind>(parser: TParser, range: TRange, kind: TKind) -> impl Parser<Output = Node>
where
    TParser: Parser<Output = Node>,
    TRange: RangeBounds<usize>,
    TKind: Into<String>
{
    let kind = kind.into();
    let kc = kind.clone();
    RepeatParser::range(parser, range).map(|_| kc, move |v| Node::branch(kind.clone(), v))
}
