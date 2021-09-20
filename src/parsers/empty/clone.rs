use super::emptyparser::EmptyParser;

impl<TOutput, FOutputFactory> Clone for EmptyParser<TOutput, FOutputFactory>
where
    TOutput: Send + Sync,
    FOutputFactory: (Fn() -> TOutput) + Clone + Send + Sync,
{
    fn clone(&self) -> Self {
        EmptyParser {
            factory: self.factory.clone(),
        }
    }
}
