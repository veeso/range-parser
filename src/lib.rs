//! # Range Parser
//!
//! range-parser is a simple Rust crate to parse range from text representation (e.g. `1-3,5-8`, `1,3,4`, `1-5`)
//! into a Vector containing all the items for that range.
//!
//! ## Get started
//!
//! First add range-parser to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! range-parser = "0.1.0"
//! ```
//!
//! Then parse a range from a string:
//!
//! ```rust
//! let range_str = "1-3,5-8";
//! let range: Vec<u64> = range_parser::parse(range_str).unwrap();
//! assert_eq!(&range, &[1, 2, 3, 5, 6, 7, 8]);
//! ```
//!
//! ## Examples
//!
//! ### Parse a range with a dash
//!
//! ```rust
//! let range: Vec<u64> = range_parser::parse("1-3").unwrap();
//! assert_eq!(range, vec![1, 2, 3]);
//! ```
//!
//! ### Parse a range with commas
//!
//! ```rust
//! let range: Vec<u64> = range_parser::parse("1,3,4").unwrap();
//! assert_eq!(range, vec![1, 3, 4]);
//! ```
//!
//! ### Parse a mixed range
//!
//! ```rust
//! let range: Vec<u64> = range_parser::parse("1,3-5,2").unwrap();
//! assert_eq!(range, vec![1, 3, 4, 5, 2]);
//! ```
//!
//! ### Parse a range with negative numbers
//!
//! ```rust
//! let range: Vec<i32> = range_parser::parse("-8,-5--1,0-3,-1").unwrap();
//! assert_eq!(range, vec![-8, -5, -4, -3, -2, -1, 0, 1, 2, 3, -1]);
//! ```
//!
//! ### Parse a range with custom separators
//!
//! ```rust
//! let range: Vec<i32> = range_parser::parse_with("-2;0..3;-1;7", ";", "..").unwrap();
//! assert_eq!(range, vec![-2, 0, 1, 2, 3, -1, 7]);
//! ```
//!

mod unit;

use std::cmp::{PartialEq, PartialOrd};
use std::ops::Add;
use std::str::FromStr;

use thiserror::Error;

pub use self::unit::Unit;

/// Parse error
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum RangeError {
    #[error("Invalid range syntax: {0}")]
    InvalidRangeSyntax(String),
    #[error("Not a number: {0}")]
    NotANumber(String),
    #[error("Value and range separators cannot be the same")]
    SeparatorsMustBeDifferent,
    #[error("Start of the range cannot be bigger than the end: {0}")]
    StartBiggerThanEnd(String),
}

/// Parse result
pub type RangeResult<T> = Result<T, RangeError>;

/// Parse a range string to a vector of usize
///
/// # Arguments
/// - range_str: &str - the range string to parse
///
/// # Returns
/// - Result<Vec<T>, RangeError> - the parsed range
///
/// # Example
///
/// ```rust
/// let range: Vec<u64> = range_parser::parse::<u64>("0-3").unwrap();
/// assert_eq!(range, vec![0, 1, 2, 3]);
///
/// let range: Vec<u64> = range_parser::parse::<u64>("0,1,2,3").unwrap();
/// assert_eq!(range, vec![0, 1, 2, 3]);
///
/// let range: Vec<i32> = range_parser::parse::<i32>("0,3,5-8,-1").unwrap();
/// assert_eq!(range, vec![0, 3, 5, 6, 7, 8, -1]);
/// ```
pub fn parse<T>(range_str: &str) -> RangeResult<Vec<T>>
where
    T: FromStr + Add<Output = T> + PartialEq + PartialOrd + Unit + Copy,
{
    let mut range = Vec::new();

    for part in range_str.split(',') {
        parse_part(&mut range, part, "-")?;
    }

    Ok(range)
}

/// Parse a range string to a vector of usize with custom separators
///
/// # Arguments
/// - range_str: &str - the range string to parse
/// - value_separator: char - the separator for single values
/// - range_separator: char - the separator for ranges
///
/// # Returns
/// - Result<Vec<T>, RangeError> - the parsed range
///
/// # Example
///
/// ```rust
/// let range: Vec<i32> = range_parser::parse_with::<i32>("0;3;5..8;-1", ";", "..").unwrap();
/// assert_eq!(range, vec![0, 3, 5, 6, 7, 8, -1]);
/// ```
pub fn parse_with<T>(
    range_str: &str,
    value_separator: &str,
    range_separator: &str,
) -> RangeResult<Vec<T>>
where
    T: FromStr + Add<Output = T> + PartialEq + PartialOrd + Unit + Copy,
{
    if value_separator == range_separator {
        return Err(RangeError::SeparatorsMustBeDifferent);
    }

    let mut range = Vec::new();

    for part in range_str.split(value_separator) {
        parse_part(&mut range, part, range_separator)?;
    }

    Ok(range)
}

/// Parse a range part to a vector of T
fn parse_part<T>(acc: &mut Vec<T>, part: &str, range_separator: &str) -> RangeResult<()>
where
    T: FromStr + Add<Output = T> + PartialEq + PartialOrd + Unit + Copy,
{
    if part.contains(range_separator) {
        parse_value_range(acc, part, range_separator)?;
    } else {
        acc.push(parse_as_t(part)?);
    }
    Ok(())
}

/// Parse value range to a vector of T
///
/// If the range is `1-3`, it will add 1, 2, 3 to the accumulator.
/// If the range starts with `-`, but has not a number before it, it will consider it as a negative number.
fn parse_value_range<T>(acc: &mut Vec<T>, part: &str, range_separator: &str) -> RangeResult<()>
where
    T: FromStr + Add<Output = T> + PartialEq + PartialOrd + Unit + Copy,
{
    let parts: Vec<&str> = part.split(range_separator).collect();

    // here it gets a bit tricky
    // because for example we could have `-1-3` which is a valid range
    // or `-5--3` which is also a valid range. So we need to find a way to tell what is dividing the range exactly
    // so let's calculate the first part index
    let (start, end): (T, T) = match parts.len() {
        2 if parts[0].is_empty() => {
            // if the first part is empty, it means it's a negative number
            let end = format!("-{}", parts[1]);
            let end: T = parse_as_t(&end)?;
            acc.push(end);
            return Ok(());
        }
        // 2 positive numbers
        2 => {
            let start = parts[0];
            let end = parts[1];
            let start: T = parse_as_t(start)?;
            let end: T = parse_as_t(end)?;
            (start, end)
        }
        // 3 is tricky, because it could be both `-1-2` or `1--3`, but the second case is invalid actually,
        // because start cannot be greater than end
        3 if parts[0].is_empty() => {
            let start = format!("-{}", parts[1]);
            let end = parts[2];
            let start: T = parse_as_t(&start)?;
            let end: T = parse_as_t(end)?;
            (start, end)
        }
        3 => return Err(RangeError::StartBiggerThanEnd(part.to_string())),
        4 => {
            let start = format!("-{}", parts[1]);
            let end = format!("-{}", parts[3]);
            let start: T = parse_as_t(&start)?;
            let end: T = parse_as_t(&end)?;
            (start, end)
        }
        _ => return Err(RangeError::InvalidRangeSyntax(part.to_string())),
    };

    // if start is bigger than end, it's an invalid range
    if start > end {
        return Err(RangeError::StartBiggerThanEnd(part.to_string()));
    }

    let mut x = start;
    while x <= end {
        acc.push(x);
        x = x + T::unit();
    }

    Ok(())
}

/// Parse a string to a T
fn parse_as_t<T>(part: &str) -> RangeResult<T>
where
    T: FromStr + Add<Output = T> + PartialEq + PartialOrd + Unit + Copy,
{
    part.trim()
        .parse()
        .map_err(|_| RangeError::NotANumber(part.to_string()))
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn should_parse_dashed_range_with_positive_numbers() {
        let range: Vec<u64> = parse("1-3").unwrap();
        assert_eq!(range, vec![1, 2, 3]);
    }

    #[test]
    fn should_parse_dashed_range_with_mixed_numbers() {
        let range: Vec<i32> = parse("-2-3").unwrap();
        assert_eq!(range, vec![-2, -1, 0, 1, 2, 3]);
    }

    #[test]
    fn should_parse_dashed_range_with_negative_numbers() {
        let range: Vec<i32> = parse("-3--1").unwrap();
        assert_eq!(range, vec![-3, -2, -1]);
    }

    #[test]
    fn should_parse_range_with_floats() {
        let range: Vec<f64> = parse("-1.0-3.0").unwrap();
        assert_eq!(range, vec![-1.0, 0.0, 1.0, 2.0, 3.0]);
    }

    #[test]
    fn should_parse_range_with_commas_with_positive_numbers() {
        let range: Vec<u64> = parse("1,3,4").unwrap();
        assert_eq!(range, vec![1, 3, 4]);
    }

    #[test]
    fn should_parse_range_with_commas_with_mixed_numbers() {
        let range: Vec<i32> = parse("-2,0,3,-1").unwrap();
        assert_eq!(range, vec![-2, 0, 3, -1]);
    }

    #[test]
    fn should_parse_mixed_range_with_positive_numbers() {
        let range: Vec<u64> = parse("1,3-5,2").unwrap();
        assert_eq!(range, vec![1, 3, 4, 5, 2]);
    }

    #[test]
    fn should_parse_mixed_range_with_mixed_numbers() {
        let range: Vec<i32> = parse("-2,0-3,-1,7").unwrap();
        assert_eq!(range, vec![-2, 0, 1, 2, 3, -1, 7]);
    }

    #[test]
    fn test_should_parse_with_whitespaces() {
        let range: Vec<u64> = parse(" 1 , 3 - 5 , 2 ").unwrap();
        assert_eq!(range, vec![1, 3, 4, 5, 2]);
    }

    #[test]
    fn should_parse_mixed_range_with_mixed_numbers_with_custom_separators() {
        let range: Vec<i32> = parse_with("-2;0..3;-1;7", ";", "..").unwrap();
        assert_eq!(range, vec![-2, 0, 1, 2, 3, -1, 7]);
    }

    #[test]
    fn test_should_not_allow_invalid_range() {
        let range = parse::<i32>("1-3-5");
        assert!(range.is_err());
    }

    #[test]
    fn test_should_not_allow_invalid_range_with_custom_separators() {
        let range = parse_with::<i32>("1-3-5", "-", "-");
        assert!(range.is_err());
    }

    #[test]
    fn test_should_not_allow_start_bigger_than_end() {
        let range = parse::<i32>("3-1");
        assert!(range.is_err());
    }
}
