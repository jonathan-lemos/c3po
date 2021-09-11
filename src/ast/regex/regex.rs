use crate::parsers::regex::regexparser::RegexParser;
use crate::parser::parser::Parser;
use super::super::node::node::Node;
use crate::parsers::map::map::Map;

pub fn regex<S: Into<String>>(pattern: S) -> impl Parser<Node> {
    let rep = RegexParser::new(pattern);
    let kind = rep.kind().to_string();
    rep.map(move |s| Node::leaf(kind.clone(), s))
}
