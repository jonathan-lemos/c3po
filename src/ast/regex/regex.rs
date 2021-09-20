use super::super::node::node::Node;
use crate::parser::parser::Parser;
use crate::parsers::map::map::Map;
use crate::parsers::regex::regexparser::RegexParser;

pub fn regex<TPattern: Into<String>, TKind: Into<String>>(
    pattern: TPattern,
    kind: TKind,
) -> impl Parser<Output = Node> {
    let rep = RegexParser::new(pattern);
    let kind = kind.into();
    let kc = kind.clone();
    rep.map(|_| kc, move |s| Node::leaf(kind.clone(), s))
}
