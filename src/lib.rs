// Copyright 2012-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Determine displayed width of `char` and `str` types according to
//! [Unicode Standard Annex #11](http://www.unicode.org/reports/tr11/)
//! rules.
//!
//! ```rust
//! extern crate unicode_width;
//!
//! use unicode_width::UnicodeWidthStr;
//!
//! fn main() {
//!     let teststr = "Ｈｅｌｌｏ, ｗｏｒｌｄ!";
//!     let width = UnicodeWidthStr::width(teststr);
//!     println!("{}", teststr);
//!     println!("The above string is {} columns wide.", width);
//!     let width = teststr.width_cjk();
//!     println!("The above string is {} columns wide (CJK).", width);
//! }
//! ```
//!
//! # features
//!
//! unicode-width does not depend on `std`, so it can be used in crates
//! with the `#![no_std]` attribute.
//!
//! # crates.io
//!
//! You can use this package in your project by adding the following
//! to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! unicode-width = "0.1.5"
//! ```

#![deny(missing_docs, unsafe_code)]
#![doc(
    html_logo_url = "https://unicode-rs.github.io/unicode-rs_sm.png",
    html_favicon_url = "https://unicode-rs.github.io/unicode-rs_sm.png"
)]
#![cfg_attr(feature = "bench", feature(test))]
#![no_std]

#[cfg(test)]
#[macro_use]
extern crate std;

#[cfg(feature = "bench")]
extern crate test;

use tables::charwidth as cw;
pub use tables::UNICODE_VERSION;

mod tables;

#[cfg(test)]
mod tests;

/// Methods for determining displayed width of Unicode characters.
pub trait UnicodeWidthChar {
    /// Returns the character's displayed width in columns, or `None` if the
    /// character is a control character other than `'\x00'`.
    ///
    /// This function treats characters in the Ambiguous category according
    /// to [Unicode Standard Annex #11](http://www.unicode.org/reports/tr11/)
    /// as 1 column wide. This is consistent with the recommendations for non-CJK
    /// contexts, or when the context cannot be reliably determined.
    fn width(self) -> Option<usize>;

    /// Returns the character's displayed width in columns, or `None` if the
    /// character is a control character other than `'\x00'`.
    ///
    /// This function treats characters in the Ambiguous category according
    /// to [Unicode Standard Annex #11](http://www.unicode.org/reports/tr11/)
    /// as 2 columns wide. This is consistent with the recommendations for
    /// CJK contexts.
    fn width_cjk(self) -> Option<usize>;
}

impl UnicodeWidthChar for char {
    #[inline]
    fn width(self) -> Option<usize> {
        cw::width(self, false)
    }

    #[inline]
    fn width_cjk(self) -> Option<usize> {
        cw::width(self, true)
    }
}

/// Methods for determining displayed width of Unicode strings.
pub trait UnicodeWidthStr {
    /// Returns the string's displayed width in columns.
    ///
    /// Control characters are treated as having zero width.
    ///
    /// This function treats characters in the Ambiguous category according
    /// to [Unicode Standard Annex #11](http://www.unicode.org/reports/tr11/)
    /// as 1 column wide. This is consistent with the recommendations for
    /// non-CJK contexts, or when the context cannot be reliably determined.
    ///
    /// Also consistent with UAX11, this function treats [emoji presentation sequences]
    /// (https://www.unicode.org/reports/tr51/#def_emoji_presentation_sequence)
    /// as 2 columns wide. This means that the width of a string may not equal
    /// the sum of the widths of its individual characters.
    fn width(&self) -> usize;

    /// Returns the string's displayed width in columns.
    ///
    /// Control characters are treated as having zero width.
    ///
    /// This function treats characters in the Ambiguous category according
    /// to [Unicode Standard Annex #11](http://www.unicode.org/reports/tr11/)
    /// as 2 column wide. This is consistent with the recommendations for
    /// CJK contexts.
    ///
    /// Also consistent with UAX11, this function treats [emoji presentation sequences]
    /// (https://www.unicode.org/reports/tr51/#def_emoji_presentation_sequence)
    /// as 2 columns wide. This means that the width of a string may not equal
    /// the sum of the widths of its individual characters.
    fn width_cjk(&self) -> usize;
}

impl UnicodeWidthStr for str {
    #[inline]
    fn width(&self) -> usize {
        str_width(self, false)
    }

    #[inline]
    fn width_cjk(&self) -> usize {
        str_width(self, true)
    }
}

fn str_width(s: &str, is_cjk: bool) -> usize {
    s.chars()
        .rfold((0, false), |(sum, was_fe0f), c| {
            if c == '\u{FE0F}' {
                (sum, true)
            } else {
                let add = if was_fe0f && cw::starts_emoji_presentation_seq(c) {
                    2
                } else {
                    cw::width(c, is_cjk).unwrap_or(0)
                };
                (sum + add, false)
            }
        })
        .0
}
