# erajp

Japanese era converter for Rust.

Inspired by [mattn/go-erajp](https://github.com/mattn/go-erajp).

## Examples

Convert a year to Japanese era.

```rust
assert_eq!(Some("平成"), erajp::to_era_from_year(2019));
```

Convert a day to Japanese era (using [chrono](https://crates.io/crates/chrono))

```rust
extern crate chrono;
use chrono::prelude::*;

let today = Local::today();
assert_eq!(Some("平成"), erajp::to_era(&today));
```

## License

MIT

## Author

Tomochika Hara (a.k.a. thara)
