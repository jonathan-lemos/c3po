use c3po_parser_macro::parser;

/// A parser that doesn't consume any input and just returns a value.
#[parser(TOutput)]
pub struct EmptyParser<TOutput, FOutputFactory>
where
    TOutput: Send + Sync,
    FOutputFactory: (Fn() -> TOutput) + Clone + Send + Sync,
{
    pub(super) factory: FOutputFactory,
}

impl<TOutput, FOutputFactory> EmptyParser<TOutput, FOutputFactory>
where
    TOutput: Send + Sync,
    FOutputFactory: (Fn() -> TOutput) + Clone + Send + Sync,
{
    /// Creates an EmptyParser that returns values from a factory function.
    pub fn using_factory(factory: FOutputFactory) -> Self {
        EmptyParser { factory }
    }
}

impl EmptyParser<(), fn() -> ()> {
    /// Creates an EmptyParser that returns the unit value (`()`).
    pub fn new() -> Self {
        EmptyParser { factory: || () }
    }
}
