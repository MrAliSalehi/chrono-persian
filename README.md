# chrono-persian
## About
this crate contains a set of helper functions to convert chrono datetimes to persian (jalali) calender
it provides a simple Trait `ToPersian` which is implemented for `NaiveDateTime`, `DateTime<Utc>` and `DateTime<Local`
##

## Example
```rust
use chrono::{DateTime, Utc, Local, NaiveDateTime};
use chrono_persian::ToPersian;
 
// convert a datetime utc
let utc = "2024-11-09 22:38:28 UTC".parse::<DateTime<Utc>>().unwrap();
let a = utc.to_persian().unwrap();
assert_eq!(a.to_string(), "1403-08-20 02:08:28 UTC");

//convert a datetime local
let local = "2024-11-10 02:17:54 +03:30".parse::<DateTime<Local>>().unwrap();
let b = local.to_persian().unwrap();
assert_eq!(b.to_string(), "1403-08-20 02:17:54 +00:00");
   
//convert a naivedatetime
let now = NaiveDateTime::parse_from_str("2024-11-09 23:07:00","%Y-%m-%d %H:%M:%S").unwrap();
let a = now.to_persian().unwrap();
assert_eq!(a.to_string(),"1403-08-19 23:07:00");
 
 ```