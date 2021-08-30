use super::super::super::immut_iterable::ImmutableIterable;
use super::super::immut_iterator::ImmutableIterator;
use super::immut_filter::ImmutableFilter;

impl<TInput: ImmutableIterable> ImmutableIterator<TInput> {
    /// Yields only the elements of the iterator for which a `filter` function returns `true`.
    /// 
    /// # Arguments
    /// * `filter` - A function that returns `true` to include an element, `false` to not.
    pub fn filter<F: (Fn(&TInput) -> bool) + Clone, TOutput>(self, filter: F) -> ImmutableIterator<ImmutableFilter<TInput, F>> {
        ImmutableIterator::new(ImmutableFilter {
            filter,
            iterator: self
        })
    }
}

