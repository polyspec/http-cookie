//! The `Cookie` field of a request read into an ordered map of names and values, and the field value of one
//! `Set-Cookie` field line. The rules are those of `docs/spec/cookies.md`; each item names its rule.
//!
//! Both operations are pure functions (CK-1.2): they read no clock, no file, no environment and no random source.

mod error;
mod parse;
mod set_cookie;

pub use error::Error;
pub use parse::parse_cookie;
pub use set_cookie::{set_cookie, Attributes, SameSite, MAX_NAME_AND_VALUE, MAX_PATH};
