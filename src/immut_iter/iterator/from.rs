use super::super::immut_iterable::ImmutableIterable;
use super::immut_iterator::ImmutableIterator;

impl<I: ImmutableIterable> From<&I> for ImmutableIterator<I> {
    fn from(iterable: &I) -> Self {
        iterable.iter_immut()
    }
}

impl<I: ImmutableIterable> From<I> for ImmutableIterator<I> {
    fn from(iterable: I) -> Self {
        ImmutableIterator::new(iterable)
    }
}

impl<I: ImmutableIterable> From<Option<I>> for ImmutableIterator<I> {
    fn from(option: Option<I>) -> Self {
        ImmutableIterator::from_option(option)
    }
}
