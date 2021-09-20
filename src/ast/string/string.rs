use super::super::node::node::Node;
use crate::parser::parser::Parser;
use crate::parsers::map::map::Map;
use crate::parsers::string::stringparser::StringParser;

pub fn string<TValue: Into<String>, TKind: Into<String>>(
    string: TValue,
    kind: TKind,
) -> impl Parser<Output = Node> {
    let kind = kind.into();
    let kc = kind.clone();
    StringParser::new(string).map(|_| kc, move |s| Node::leaf(kind.clone(), s))
}
