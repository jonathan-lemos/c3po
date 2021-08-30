#[macro_use]
extern crate static_assertions;

pub mod parser;
pub mod immut_iter;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
