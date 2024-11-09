//! # chrono-persian
//! ## About
//! this crate contains a set of helper functions to convert chrono datetimes to persian (jalali) calender
//! it provides a simple Trait `ToPersian` which is implemented for `NaiveDateTime`, `DateTime<Utc>` and `DateTime<Local`
//! ##
//! 
//! ## Example
//! ```
//!use chrono::{DateTime, Utc, Local, NaiveDateTime};
//!use chrono_persian::ToPersian;
//! 
//!// convert a datetime utc
//!let utc = "2024-11-09 22:38:28 UTC".parse::<DateTime<Utc>>().unwrap();
//!let a = utc.to_persian().unwrap();
//!assert_eq!(a.to_string(), "1403-08-20 02:08:28 UTC");
//!
//!//convert a datetime local
//!let local = "2024-11-10 02:17:54 +03:30".parse::<DateTime<Local>>().unwrap();
//!let b = local.to_persian().unwrap();
//!assert_eq!(b.to_string(), "1403-08-20 02:17:54 +00:00");
//!   
//!//convert a naivedatetime
//!let now = NaiveDateTime::parse_from_str("2024-11-09 23:07:00","%Y-%m-%d %H:%M:%S").unwrap();
//!let a = now.to_persian().unwrap();
//!assert_eq!(a.to_string(),"1403-08-19 23:07:00");
//! 
//! ```

use chrono::{DateTime, Datelike, FixedOffset, Local, NaiveDate, NaiveDateTime, TimeZone, Utc};
use std::ops::Deref;
use std::sync::LazyLock;




/// Iran's offset, already tested, so its safe to unwrap
static LOCAL: LazyLock<Local> = LazyLock::new(|| unsafe {
    Local::from_offset(&FixedOffset::east_opt(3 * 3600 + 1800).unwrap_unchecked())
});
static ZERO_OFFSET: LazyLock<FixedOffset> =
    LazyLock::new(|| unsafe { FixedOffset::east_opt(0).unwrap_unchecked() });

/// Convert a chrono type to the persian equivalent
pub trait ToPersian {
    fn to_persian(&self) -> Option<Self>
    where
        Self: Sized;
}

impl ToPersian for DateTime<Utc> {
    /// Convert a `DateTime<Utc>` to the persian equivalent
    /// ```rust
    ///use chrono::{DateTime, Utc};
    ///use chrono_persian::ToPersian;
    ///
    ///let utc = "2024-11-09 22:38:28 UTC".parse::<DateTime<Utc>>().unwrap();
    ///let a = utc.to_persian().unwrap();
    ///assert_eq!(a.to_string(), "1403-08-20 02:08:28 UTC");
    /// ```
    fn to_persian(&self) -> Option<Self> {
        let now = self.with_timezone(LOCAL.deref());
        let (y, m, d) = gregorian_to_jalali(now.year(), now.month(), now.day());
        Some(NaiveDateTime::new(NaiveDate::from_ymd_opt(y, m, d)?, now.time()).and_utc())
    }
}

impl ToPersian for DateTime<Local> {
    /// Convert a `DateTime<Local>` to the persian equivalent
    /// ```rust
    ///use chrono::{DateTime, Local};
    ///use chrono_persian::ToPersian;
    ///
    ///let local = "2024-11-10 02:17:54 +03:30".parse::<DateTime<Local>>().unwrap();
    ///let b = local.to_persian().unwrap();
    ///assert_eq!(b.to_string(), "1403-08-20 02:17:54 +00:00");
    /// ```
    fn to_persian(&self) -> Option<Self> {
        let now = self.with_timezone(LOCAL.deref());
        let (y, m, d) = gregorian_to_jalali(now.year(), now.month(), now.day());
        let a = NaiveDateTime::new(NaiveDate::from_ymd_opt(y, m, d)?, now.time());
        Some(DateTime::<Local>::from_naive_utc_and_offset(
            a,
            *ZERO_OFFSET,
        ))
    }
}

impl ToPersian for NaiveDateTime {
    /// Convert a `NaiveDateTime` to the persian equivalent
    /// ```rust
    ///use chrono::NaiveDateTime;
    ///use chrono_persian::ToPersian;
    ///
    ///let now = NaiveDateTime::parse_from_str("2024-11-09 23:07:00","%Y-%m-%d %H:%M:%S").unwrap();
    ///let a = now.to_persian().unwrap();
    ///assert_eq!(a.to_string(),"1403-08-19 23:07:00");
    /// ```
    fn to_persian(&self) -> Option<Self> {
        let now = self.and_local_timezone(*LOCAL).earliest()?;
        let (y, m, d) = gregorian_to_jalali(now.year(), now.month(), now.day());
        Some(NaiveDateTime::new(
            NaiveDate::from_ymd_opt(y, m, d)?,
            now.time(),
        ))
    }
}

/// source: https://jdf.scr.ir
fn gregorian_to_jalali(gy: i32, gm: u32, gd: u32) -> (i32, u32, u32) {
    const G_D_M: [i32; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
    let gy2 = if gm > 2 { gy + 1 } else { gy };

    let mut days = 355666 + (365 * gy) + ((gy2 + 3) / 4) - ((gy2 + 99) / 100)
        + ((gy2 + 399) / 400)
        + gd as i32
        + G_D_M[(gm - 1) as usize];

    let mut jy = -1595 + (33 * (days / 12053));
    days %= 12053;
    jy += 4 * (days / 1461);
    days %= 1461;

    if days > 365 {
        jy += (days - 1) / 365;
        days = (days - 1) % 365;
    }

    let jm = if days < 186 {
        1 + (days / 31)
    } else {
        7 + ((days - 186) / 30)
    };

    let jd = if days < 186 {
        1 + (days % 31)
    } else {
        1 + ((days - 186) % 30)
    };

    (jy, jm as u32, jd as u32)
}
