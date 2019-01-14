//! # era-jp
//!
//! converter for Japanese era
//!
//! ## Examples
//!
//! a year to Japanese era
//!
//! ```rust
//! # extern crate erajp;
//! assert_eq!(Some("平成"), erajp::to_era_from_year(2019));
//! ```
//!
//! a day to Japanese era
//!
//! ```rust
//! # extern crate erajp;
//! extern crate chrono;
//! use chrono::prelude::*;
//!
//! let today = Local::today();
//! assert_eq!(Some("平成"), erajp::to_era(&today));
//! ```
extern crate csv;

#[macro_use]
extern crate serde_derive;

extern crate serde;

extern crate chrono;
use chrono::prelude::*;

extern crate chrono_tz;
use chrono_tz::Asia::Tokyo;

#[macro_use]
extern crate lazy_static;

#[derive(Debug, Deserialize)]
struct Era {
    name: String,
    ruby: String,
    year: i32,
    month: u32,
    day: u32,
}

const ERA_TABLE: &str = include_str!("era.csv");

lazy_static! {
    static ref ERA_LIST: Vec<Era> = {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(ERA_TABLE.as_bytes());
        let mut v = Vec::new();
        for result in rdr.deserialize() {
            let rec: Era = result.unwrap();
            v.push(rec)
        }
        v.reverse();
        v
    };
}

/// Given a year, return a string of japanese era
///
/// Return `None` if the year doesn't match any eras.
pub fn to_era_from_year<'a>(year: i32) -> Option<&'a str> {
    ERA_LIST
        .iter()
        .find(|&era| era.year <= year)
        .as_ref()
        .map(|&era| era.name.as_ref())
}

/// Given a local time, return a string of japanese era
///
/// Return `None` if the time doesn't match any eras.
pub fn to_era(local: &chrono::Date<chrono::offset::Local>) -> Option<&str> {
    let dt = local.with_timezone(&Tokyo);

    ERA_LIST
        .iter()
        .find(|&era| {
            let start = Tokyo
                .ymd_opt(era.year, era.month, era.day)
                .single()
                .unwrap_or_else(|| {
                    //FIXME Avoid 'No such local times' error with 弘安, 文亀, 永正, 寛永.
                    // Maybe should use Julian calendar, but no problem in current history.
                    Tokyo.ymd(era.year, era.month, 28)
                });
            0 <= dt.signed_duration_since(start).num_days()
        })
        .as_ref()
        .map(|&era| era.name.as_ref())
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_to_era() {
        let today = Local::today();
        assert_eq!(Some("平成"), to_era(&today));

        let day = Local.ymd(1861, 2, 19);
        assert_eq!(Some("文久"), to_era(&day));

        let day = Local.ymd(1278, 3, 10);
        assert_eq!(Some("弘安"), to_era(&day));
    }

    #[test]
    fn test_to_era_from_year() {
        assert_eq!(Some("明治"), to_era_from_year(1910));
        assert_eq!(Some("明治"), to_era_from_year(1911));
        assert_eq!(Some("大正"), to_era_from_year(1912));
        assert_eq!(Some("大正"), to_era_from_year(1925));
        assert_eq!(Some("昭和"), to_era_from_year(1926));
        assert_eq!(Some("昭和"), to_era_from_year(1988));
        assert_eq!(Some("平成"), to_era_from_year(1989));
        assert_eq!(Some("平成"), to_era_from_year(2019));
    }
}
