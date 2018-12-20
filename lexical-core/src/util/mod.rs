//! Helper utilities for low-level features.
// Fix a compiler bug that thinks `pow` isn't used.
#![allow(unused_imports)]

// Hide implementation details.
#[macro_use]
pub(crate) mod api;

#[macro_use]
mod assert;

#[cfg(test)]
#[macro_use]
pub(crate) mod test;

// Hide implementation details.
mod algorithm;
mod cast;
mod config;
mod error;
mod mask;
mod num;
mod primitive;
mod parse_state;    // TODO(ahuszagh) Remove
mod pow;
mod result;
mod sign;
mod table;

cfg_if! {
if #[cfg(feature = "correct")] {
    mod slice;
    mod veclike;
} else {
    mod wrapped;
}}  // cfg_if

// Publicly export everything with crate-visibility.
pub(crate) use self::algorithm::*;
pub(crate) use self::cast::*;
pub(crate) use self::mask::*;
pub(crate) use self::num::*;
pub(crate) use self::primitive::*;
pub(crate) use self::parse_state::*;
pub(crate) use self::pow::*;
pub(crate) use self::sign::*;
pub(crate) use self::table::*;

cfg_if! {
if #[cfg(feature = "correct")] {
    pub(crate) use self::slice::*;
    pub(crate) use self::veclike::*;
} else {
    pub(crate) use self::wrapped::*;
}}  // cfg_if

// Publicly export config globally.
pub use self::config::*;
pub use self::error::{Error, ErrorCode, is_empty, is_invalid_digit, is_overflow, is_success};
pub use self::result::*;
