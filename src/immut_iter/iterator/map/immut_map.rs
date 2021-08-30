use super::super::super::immut_iterable::ImmutableIterable;
use super::super::immut_iterator::ImmutableIterator;

/// An `ImmutableIterator` adapter that returns modified elements based on a given mapping function.
pub struct ImmutableMap<TInput: ImmutableIterable, F: Fn(&TInput) -> TOutput, TOutput> {
    pub(super) mapper: F,
    pub(super) iterator: ImmutableIterator<TInput>
}
