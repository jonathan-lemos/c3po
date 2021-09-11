use super::super::node::node::Node;
use crate::parsers::map::map::Map;
use crate::parsers::map::mapparser::MapParser;
use crate::parsers::string::stringparser::StringParser;

pub fn string<S: Into<String>>(string: S) -> MapParser<String, StringParser, Node, fn(String) -> Node> {
    StringParser::new(string).map(|s| Node::leaf("string", s))
}
