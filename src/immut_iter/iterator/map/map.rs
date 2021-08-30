use super::super::super::immut_iterable::ImmutableIterable;
use super::super::immut_iterator::ImmutableIterator;
use super::immut_map::ImmutableMap;

impl<TInput: ImmutableIterable> ImmutableIterator<TInput> {
    /// Transforms the elements of the `ImmutableIterator` by applying the given `mapper` to each element.
    /// 
    /// # Arguments
    /// * `mapper` - A function that transforms input elements to output elements, yielding `mapper(x)` for each `x` in the input iterator.
    pub fn map<F: (Fn(&TInput) -> TOutput) + Clone, TOutput>(self, mapper: F) -> ImmutableIterator<ImmutableMap<TInput, F, TOutput>> {
        ImmutableIterator::new(ImmutableMap {
            mapper,
            iterator: self
        })
    }
}
