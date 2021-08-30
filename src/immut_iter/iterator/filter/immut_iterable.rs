use super::super::super::immut_iterable::ImmutableIterable;
use super::super::immut_iterator::ImmutableIterator;
use super::immut_filter::ImmutableFilter;

impl<TInput: ImmutableIterable, F: (Fn(&TInput) -> bool) + Clone> ImmutableIterable
    for ImmutableFilter<TInput, F>
{
    fn next_immut(&self) -> Option<Self> {
        match &self.iterator.iterable {
            None => None,
            Some(s) => s.next_immut().map(|s2| ImmutableFilter {
                filter: self.filter.clone(),
                iterator: ImmutableIterator::new(s2),
            }),
        }
    }
}