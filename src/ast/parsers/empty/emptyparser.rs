/// A parser that doesn't consume any input and just returns a value.
#[derive(Debug)]
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

impl<TOutput> EmptyParser<TOutput, fn() -> TOutput>
where
    TOutput: Default + Send + Sync,
{
    /// Creates an EmptyParser that returns the `Default` value for a type.
    pub fn new() -> Self {
        EmptyParser {
            factory: || TOutput::default(),
        }
    }
}
