use super::super::super::immut_iterable::ImmutableIterable;
use super::super::immut_iterator::ImmutableIterator;
use super::immut_map::ImmutableMap;

impl<TInput: ImmutableIterable, F: (Fn(&TInput) -> TOutput) + Clone, TOutput> ImmutableIterable
    for ImmutableMap<TInput, F, TOutput>
{
    fn next_immut(&self) -> Option<Self> {
        match &self.iterator.iterable {
            None => None,
            Some(s) => s.next_immut().map(|s2| ImmutableMap {
                mapper: self.mapper.clone(),
                iterator: ImmutableIterator::new(s2),
            }),
        }
    }
}
