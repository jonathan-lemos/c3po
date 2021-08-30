use super::iterator::immut_iterator::ImmutableIterator;

/// Can retrieve the next element of a sequence with an immutable reference.
/// The output type must be `Self`.
/// 
/// This is meant to be the immutable reference equivalent of `Iterator::next(&mut Self)`
pub trait ImmutableIterable: Clone + Sized {
    fn next_immut(&self) -> Option<Self>;

    fn iter_immut(&self) -> ImmutableIterator<Self> {
        ImmutableIterator::new(self.clone())
    }
}
