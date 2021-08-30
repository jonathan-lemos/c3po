use super::super::super::immut_iterable::ImmutableIterable;
use super::super::immut_iterator::ImmutableIterator;

/// An `ImmutableIterator` adapter that filters out elements for which a given function returns `false`.
pub struct ImmutableFilter<TInput: ImmutableIterable, F: Fn(&TInput) -> bool> {
    pub(super) filter: F,
    pub(super) iterator: ImmutableIterator<TInput>
}
