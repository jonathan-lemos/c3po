use c3po::parsers::map::map::Map;
use c3po::parser::parser::Parser;
use c3po::parsers::regex::regexparser::RegexParser;

pub struct Ident(String);

pub fn ident() -> impl Parser<Output = Ident> {
    return RegexParser::new("[a-zA-Z0-9_-]+").map(|_| "identifier", Ident)
}
