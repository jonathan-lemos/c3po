use super::super::immut_iterable::ImmutableIterable;
use super::immut_iterator::ImmutableIterator;

impl<I: ImmutableIterable> Iterator for ImmutableIterator<I> {
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.iterable {
            None => None,
            Some(v) => {
                match v.next_immut() {
                    None => self.iterable.take(),
                    Some(next) => self.iterable.replace(next)
                }
            }
        }
    }
}
