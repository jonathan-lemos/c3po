use super::super::immut_iterable::ImmutableIterable;
use super::immut_iterator::ImmutableIterator;

impl<'a, I: ImmutableIterable> Iterator for ImmutableIterator<'a, I> {
    type Item = &'a I;

    fn next(&'a mut self) -> Option<Self::Item> {
        let ret = match &self {
            ImmutableIterator::Empty => return None,
            ImmutableIterator::Owned(v) => v,
            ImmutableIterator::Reference(r) => *r
        };

        *self = match ret.next_immut() {
            Some(next) => ImmutableIterator::Owned(next),
            None => ImmutableIterator::Empty
        };

        Some(ret)
    }
}
