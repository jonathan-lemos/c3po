use c3po::parsers::string::stringparser::StringParser;
use c3po::parser::parser::Parser;
use c3po::parsers::map::map::Map;

pub struct Let;

pub fn r#let() -> impl Parser<Output = Let> {
    StringParser::new("let").map(|_| "let", |_| Let)
}