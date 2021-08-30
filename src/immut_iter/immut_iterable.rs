use super::iterator::immut_iterator::ImmutableIterator;

/// Can retrieve the next element with an immutable reference.
/// 
/// This is meant to be the immutable reference equivalent of `Iterator::next(&mut Self)`
/// 
/// The output type must be `Self`.
pub trait ImmutableIterable: Sized {
    fn next_immut(&self) -> Option<Self>;

    fn iter_immut(&self) -> ImmutableIterator<Self> {
        ImmutableIterator::new(self)
    }
}
