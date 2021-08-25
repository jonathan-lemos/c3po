use std::ops::BitOr;

pub struct Parser<T> {
    func: Box<dyn Fn(&str) -> Option<T>>
}

impl<T> BitOr for Parser<T> {
    type Output = Parser<T>;

    fn bitor(self, rhs: Self) -> Self::Output {
        Parser {
            func: Box::new(|s| self.func(s).or_else(rhs.func(s))) 
        }
    }
}