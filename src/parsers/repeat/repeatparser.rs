use crate::parser::parser::Parser;
use std::marker::PhantomData;
use std::ops::Bound;
use std::ops::RangeBounds;

/// Matches a Parser 0 or more times.
#[derive(Debug)]
pub struct RepeatParser<TOutput, TParser>
where
    TOutput: Send + Sync,
    TParser: Parser<Output = TOutput>,
{
    // bounds are inclusive
    pub(super) bounds: Option<(usize, Option<usize>)>,
    pub(super) parser: TParser,
    pub(super) kind: String,
    pub(super) o: PhantomData<TOutput>,
}

// the usizes are inclusive
fn range_to_values<TRange: RangeBounds<usize>>(
    range: &TRange,
) -> Option<(usize, Option<usize>)> {
    let lb = match range.start_bound() {
        Bound::Included(n) => *n,
        Bound::Excluded(n) => n + 1,
        Bound::Unbounded => 0,
    };

    let rb = match range.end_bound() {
        Bound::Included(n) => Some(*n),
        Bound::Excluded(n) => {
            if *n == 0 || *n <= lb {
                return None;
            } else {
                Some(n - 1)
            }
        }
        Bound::Unbounded => None,
    };
    Some((lb, rb))
}

fn format_range<TRange: RangeBounds<usize>>(range: &TRange) -> String {
    let (lb, rb) = match range_to_values(range) {
        Some((lb, rb)) => (lb, rb),
        None => return "never".to_string(),
    };

    match rb {
        Some(rb) => {
            if lb == 0 {
                format!("<={}", rb)
            } else if lb == rb {
                lb.to_string()
            } else {
                format!("{}-{}", lb, rb)
            }
        }
        None => {
            if lb == 0 {
                "any".to_string()
            } else {
                format!(">={}", lb)
            }
        }
    }
}

impl<TOutput, TParser> RepeatParser<TOutput, TParser>
where
    TOutput: Send + Sync,
    TParser: Parser<Output = TOutput>,
{
    /// Creates a new RepeatParser that matches a `parser` `range` times.
    ///
    /// # Arguments
    /// * `parser` - The parser to repeat.
    /// * `range`  - The amount of times to match `parser`. For example, `3..6` will match the parser 3 to 5 times.
    pub fn range<TRange: RangeBounds<usize>>(parser: TParser, range: TRange) -> Self {
        let kind = format!("({}) * {}", parser.kind(), format_range(&range));
        let bounds = range_to_values(&range);

        RepeatParser {
            o: PhantomData,
            bounds,
            parser,
            kind,
        }
    }

    /// Creates a new RepeatParser that matches a `parser` `range` times.
    ///
    /// # Arguments
    /// * `parser` - The parser to repeat.
    /// * `count`  - The amount of times to match `parser`.
    pub fn count(parser: TParser, count: usize) -> Self {
        Self::range(parser, count..=count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_range_is_invalid() {
        let result = range_to_values(&(2..2));
        assert_eq!(result, None);
    }

    #[test]
    fn negative_range_is_invalid() {
        let result = range_to_values(&(2..1));
        assert_eq!(result, None);
    }

    #[test]
    fn zero_range_doesnt_throw() {
        let result = range_to_values(&(0..0));
        assert_eq!(result, None);
    }

    #[test]
    fn unbounded_upper_range_has_none_for_second() {
        let result = range_to_values(&(5..));
        assert_eq!(result, Some((5, None)));
    }

    #[test]
    fn bounded_range_has_values_for_both() {
        let result = range_to_values(&(5..=10));
        assert_eq!(result, Some((5, Some(10))));
    }

    #[test]
    fn unbounded_lower_range_has_zero_for_first() {
        let result = range_to_values(&(..10));
        assert_eq!(result, Some((0, Some(9))));
    }

    #[test]
    fn infinite_range_is_zero_to_none() {
        let result = range_to_values(&(..));
        assert_eq!(result, Some((0, None)));
    }


    #[test]
    fn infinite_range_prints_any() {
        let result = format_range(&(..));
        assert_eq!(&result, "any");
    }

    #[test]
    fn infinite_range_prints_any_2() {
        let result = format_range(&(0..));
        assert_eq!(&result, "any");
    }


    #[test]
    fn unbounded_lower_range_prints_lte_n() {
        let result = format_range(&(..=10));
        assert_eq!(&result, "<=10");
    }


    #[test]
    fn unbounded_upper_range_prints_gte_n() {
        let result = format_range(&(10..));
        assert_eq!(&result, ">=10");
    }

    #[test]
    fn bounded_range_prints_range() {
        let result = format_range(&(1..10));
        assert_eq!(&result, "1-9");
    }

    #[test]
    fn invalid_range_prints_never() {
        let result = format_range(&(1..1));
        assert_eq!(&result, "never");
    }
}
