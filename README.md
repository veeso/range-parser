# range-parser

[![license-mit](https://img.shields.io/badge/License-MIT-teal.svg)](https://opensource.org/license/mit/)
[![build-test](https://github.com/veeso/range-parser/actions/workflows/build-test.yml/badge.svg)](https://github.com/veeso/range-parser/actions/workflows/build-test.yml)
[![downloads](https://img.shields.io/crates/d/range-parser.svg)](https://crates.io/crates/range-parser)
[![latest version](https://img.shields.io/crates/v/range-parser.svg)](https://crates.io/crates/range-parser)
[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-%23FE5196?logo=conventionalcommits&logoColor=white)](https://conventionalcommits.org)
[![docs](https://docs.rs/range-parser/badge.svg)](https://docs.rs/range-parser)

---

- [range-parser](#range-parser)
  - [About range-parser](#about-range-parser)
  - [Get started](#get-started)
  - [Supported types](#supported-types)
    - [range-parser for custom types](#range-parser-for-custom-types)
  - [Examples](#examples)
    - [Parse a range with a dash](#parse-a-range-with-a-dash)
    - [Parse a range with commas](#parse-a-range-with-commas)
    - [Parse a mixed range](#parse-a-mixed-range)
    - [Parse a range with negative numbers](#parse-a-range-with-negative-numbers)
    - [Parse a range with custom separators](#parse-a-range-with-custom-separators)
  - [Changelog](#changelog)
  - [License](#license)

---

## About range-parser

range-parser is a simple Rust crate to parse range from text representation (e.g. `1-3,5-8`, `1,3,4`, `1-5`) into a Vector containing all the items for that range.

## Get started

1. Include `range-parser` to your `Cargo.toml`

    ```toml
    range-parser = "0.1"
    ```

2. Parse range from str

    ```rust
    let range_str = "1-3,5-8";
    let range: Vec<u64> = range_parser::parse(range_str).unwrap();

    assert_eq!(&range, &[1, 2, 3, 5, 6, 7, 8]);
    ```

## Supported types

range-parser supports any kind of number primitive.

### range-parser for custom types

It is possible to extend the range-parser for custom types as long as they satisfy these trait bounds: `T: FromStr + Add<Output = T> + PartialEq + PartialOrd + Unit + Copy,`.

This requires you to implement the trait `Unit` which is exposed by this library.

The trait **Unit** is defined as

```rust
pub trait Unit {
    fn unit() -> Self;
}
```

and should return the base unit for a type, which for numbers should be `1`.

## Examples

### Parse a range with a dash

```rust
let range: Vec<u64> = range_parser::parse("1-3").unwrap();
assert_eq!(range, vec![1, 2, 3]);
```

### Parse a range with commas

```rust
let range: Vec<u64> = range_parser::parse("1,3,4").unwrap();
assert_eq!(range, vec![1, 3, 4]);
```

### Parse a mixed range

```rust
let range: Vec<u64> = range_parser::parse("1,3-5,2").unwrap();
assert_eq!(range, vec![1, 3, 4, 5, 2]);
```

### Parse a range with negative numbers

```rust
let range: Vec<i32> = range_parser::parse("-8,-5--1,0-3,-1").unwrap();
assert_eq!(range, vec![-8, -5, -4, -3, -2, -1, 0, 1, 2, 3, -1]);
```

### Parse a range with custom separators

```rust
// parse range using `;` as separator for values and `..` as separator for ranges
let range: Vec<i32> = range_parser::parse_with("-2;0..3;-1;7", ";", "..").unwrap();
assert_eq!(range, vec![-2, 0, 1, 2, 3, -1, 7]);
```

---

## Changelog

View range-parser's changelog [HERE](CHANGELOG.md)

---

## License

range-parser is licensed under the MIT license.

You can read the entire license [HERE](LICENSE)
