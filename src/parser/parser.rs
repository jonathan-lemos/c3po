use super::cursor::cursor::Cursor;
use super::parse::parse::Parse;
use super::output::output::ParserOutput;

pub struct Parser<TOutput, TState = ()> {
    pub(crate) func: Box<dyn for<'a> Fn(Option<Cursor<'a>>, &mut TState) -> Parse<'a, TOutput>>,
    pub(crate) kind: String,
    pub(crate) state: TState
}

impl<TOutput: 'static, TState> Parser<TOutput, TState> {
    pub fn new<S: Into<String>>(kind: S, initial_state: TState, func: Box<dyn for<'a> Fn(Option<Cursor<'a>>, &mut TState) -> Parse<'a, TOutput>>) -> Self {
        Parser {
            func,
            kind: kind.into(),
            state: initial_state,
        }
    }

    pub fn parse<'a>(&mut self, cursor: Option<Cursor<'a>>) -> ParserOutput<'a, TOutput> {
        let result = (self.func)(cursor, &mut self.state);
        ParserOutput::new(result, cursor, self.kind.clone())
    }

    pub fn kind(&self) -> &str {
        &self.kind
    }

    pub fn state(&self) -> &TState {
        &self.state
    }
}

impl<TOutput: 'static> Parser<TOutput> {
    pub fn stateless<S: Into<String>>(kind: S, func: Box<dyn for<'a> Fn(Option<Cursor<'a>>) -> Parse<'a, TOutput>>) -> Self {
        Parser {
            func: Box::new(move |cursor, _| func(cursor)),
            kind: kind.into(),
            state: ()
        }
    }
}
