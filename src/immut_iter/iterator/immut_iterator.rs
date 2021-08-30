use super::super::immut_iterable::ImmutableIterable;

#[derive(Debug, Clone, Copy)]
pub enum ImmutableIterator<'a, I: ImmutableIterable> {
    Reference(&'a I),
    Owned(I),
    Empty
}

impl<'a, I: ImmutableIterable> ImmutableIterator<'a, I> {
    pub fn new(iterable: &'a I) -> Self {
        ImmutableIterator::Reference(iterable)
    }
}
