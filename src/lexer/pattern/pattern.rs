use std::fmt::Debug;

pub trait Pattern: Debug + Send + Sync {
    fn match_start(&self, input: &str) -> Option<usize>;
}