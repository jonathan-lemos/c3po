use super::super::immut_iterable::ImmutableIterable;

/// Iterator for an `ImmutableIterable`
#[derive(Debug, Clone, Copy)]
pub struct ImmutableIterator<I: ImmutableIterable> {
    pub(super) iterable: Option<I>
}

impl<I: ImmutableIterable> ImmutableIterator<I> {
    /// Creates a new `ImmutableIterator` from the given `ImmutableIterable`
    pub fn new(iterable: I) -> Self {
        ImmutableIterator {
            iterable: Some(iterable)
        }
    }

    /// Creates a new `ImmutableIterator` from the given `Option<ImmutableIterable>`. `None` will produce an empty iterator.
    pub fn from_option(option: Option<I>) -> Self {
        ImmutableIterator {
            iterable: option
        }
    }
}
