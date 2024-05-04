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
//! [Unicode Standard Annex #11](http://www.unicode.org/reports/tr11/),
//! other portions of the Unicode standard, and common implementations of
//! POSIX [`wcwidth()`](https://pubs.opengroup.org/onlinepubs/9699919799/).
//! See the [Rules for determining width](#rules-for-determining-width) section
//! for the exact rules.
//!
//! This crate is `#![no_std]`.
//!
//! ```rust
//! use unicode_width::UnicodeWidthStr;
//!
//! let teststr = "Ｈｅｌｌｏ, ｗｏｒｌｄ!";
//! let width = UnicodeWidthStr::width(teststr);
//! println!("{}", teststr);
//! println!("The above string is {} columns wide.", width);
//! let width = teststr.width_cjk();
//! println!("The above string is {} columns wide (CJK).", width);
//! ```
//!
//! # Rules for determining width
//!
//! This crate currently uses the following rules to determine the width of a
//! character or string, in order of decreasing precedence. These may be tweaked in the future.
//!
//! 1. [Emoji presentation sequences] have width 2.
//!    (The width of a string may therefore differ from the sum of the widths of its characters.)
//! 2. Outside of an East Asian context, [text presentation sequences] have width 1
//!    iff their base character fulfills all the following requirements:
//!    - Has the [`Emoji_Presentation`] property, and
//!    - Not in the [Enclosed Ideographic Supplement] block.
//! 3. [`'\u{00AD}'` SOFT HYPHEN](https://util.unicode.org/UnicodeJsps/character.jsp?a=00AD) has width 1.
//! 4. [`'\u{115F}'` HANGUL CHOSEONG FILLER](https://util.unicode.org/UnicodeJsps/character.jsp?a=115F) has width 2.
//! 5. The following have width 0:
//!    - [Characters](https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=%5Cp%7BDefault_Ignorable_Code_Point%7D)
//!       with the [`Default_Ignorable_Code_Point`](https://www.unicode.org/versions/Unicode15.0.0/ch05.pdf#G40095) property.
//!    - [Characters](https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=%5Cp%7BGrapheme_Extend%7D)
//!       with the [`Grapheme_Extend`] property.
//!    - The following 8 characters, all of which have NFD decompositions consisting of two [`Grapheme_Extend`] characters:
//!      - [`'\u{0CC0}'` KANNADA VOWEL SIGN II](https://util.unicode.org/UnicodeJsps/character.jsp?a=0CC0),
//!      - [`'\u{0CC7}'` KANNADA VOWEL SIGN EE](https://util.unicode.org/UnicodeJsps/character.jsp?a=0CC7),
//!      - [`'\u{0CC8}'` KANNADA VOWEL SIGN AI](https://util.unicode.org/UnicodeJsps/character.jsp?a=0CC8),
//!      - [`'\u{0CCA}'` KANNADA VOWEL SIGN O](https://util.unicode.org/UnicodeJsps/character.jsp?a=0CCA),
//!      - [`'\u{0CCB}'` KANNADA VOWEL SIGN OO](https://util.unicode.org/UnicodeJsps/character.jsp?a=0CCB),
//!      - [`'\u{1B3B}'` BALINESE VOWEL SIGN RA REPA TEDUNG](https://util.unicode.org/UnicodeJsps/character.jsp?a=1B3B),
//!      - [`'\u{1B3D}'` BALINESE VOWEL SIGN LA LENGA TEDUNG](https://util.unicode.org/UnicodeJsps/character.jsp?a=1B3D), and
//!      - [`'\u{1B43}'` BALINESE VOWEL SIGN PEPET TEDUNG](https://util.unicode.org/UnicodeJsps/character.jsp?a=1B43).
//!    - [Characters](https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=%5Cp%7BHangul_Syllable_Type%3DV%7D%5Cp%7BHangul_Syllable_Type%3DT%7D)
//!       with a [`Hangul_Syllable_Type`] of `Vowel_Jamo` (`V`) or `Trailing_Jamo` (`T`).
//!    - [`'\0'` NUL](https://util.unicode.org/UnicodeJsps/character.jsp?a=0000).
//! 6. The [control characters](https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=%5Cp%7BCc%7D)
//!    have no defined width, and are ignored when determining the width of a string.
//! 7. [Characters](https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=%5Cp%7BEast_Asian_Width%3DF%7D%5Cp%7BEast_Asian_Width%3DW%7D)
//!    with an [`East_Asian_Width`] of [`Fullwidth`] or [`Wide`] have width 2.
//! 8. [Characters](https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=%5Cp%7BEast_Asian_Width%3DA%7D)
//!    with an [`East_Asian_Width`] of [`Ambiguous`] have width 2 in an East Asian context, and width 1 otherwise.
//! 9. All other characters have width 1.
//!
//! [`East_Asian_Width`]: https://www.unicode.org/reports/tr11/#ED1
//! [`Emoji_Presentation`]: https://unicode.org/reports/tr51/#def_emoji_presentation
//! [`Grapheme_Extend`]: https://www.unicode.org/versions/Unicode15.0.0/ch03.pdf#G52443
//! [`Hangul_Syllable_Type`]: https://www.unicode.org/versions/Unicode15.0.0/ch03.pdf#G45593
//!
//! [`Fullwidth`]: https://www.unicode.org/reports/tr11/#ED2
//! [`Wide`]: https://www.unicode.org/reports/tr11/#ED4
//! [`Ambiguous`]: https://www.unicode.org/reports/tr11/#ED6
//!
//! [Emoji presentation sequences]: (https://unicode.org/reports/tr51/#def_emoji_presentation_sequence)
//! [text presentation sequences]: (https://unicode.org/reports/tr51/#def_text_presentation_sequence)
//!
//! [Enclosed Ideographic Supplement]: https://unicode.org/charts/PDF/U1F200.pdf
//!
//! ## Canonical equivalence
//!
//! The non-CJK width methods guarantee that canonically equivalent strings are assigned the same width.
//! However, this guarantee does not currently hold for the CJK width variants.

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![doc(
    html_logo_url = "https://unicode-rs.github.io/unicode-rs_sm.png",
    html_favicon_url = "https://unicode-rs.github.io/unicode-rs_sm.png"
)]
#![no_std]

use tables::charwidth as cw;
pub use tables::UNICODE_VERSION;

mod tables;

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
    /// Control characters are treated as having zero width,
    /// and [emoji presentation sequences](https://unicode.org/reports/tr51/#def_emoji_presentation_sequence)
    /// are assigned width 2.
    ///
    /// This function treats characters in the Ambiguous category according
    /// to [Unicode Standard Annex #11](http://www.unicode.org/reports/tr11/)
    /// as 1 column wide. This is consistent with the recommendations for
    /// non-CJK contexts, or when the context cannot be reliably determined.
    fn width(&self) -> usize;

    /// Returns the string's displayed width in columns.
    ///
    /// Control characters are treated as having zero width,
    /// and [emoji presentation sequences](https://unicode.org/reports/tr51/#def_emoji_presentation_sequence)
    /// are assigned width 2.
    ///
    /// This function treats characters in the Ambiguous category according
    /// to [Unicode Standard Annex #11](http://www.unicode.org/reports/tr11/)
    /// as 2 column wide. This is consistent with the recommendations for
    /// CJK contexts.
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum VariationSelector {
    Vs15 = 0x0E,
    Vs16 = 0x0F,
}

fn str_width(s: &str, is_cjk: bool) -> usize {
    s.chars()
        .rfold((0, None), |(sum, vsel), c| match c {
            '\u{FE0E}' => (sum, Some(VariationSelector::Vs15)),
            '\u{FE0F}' => (sum, Some(VariationSelector::Vs16)),
            _ => {
                let add = match vsel {
                    Some(VariationSelector::Vs15)
                        if !is_cjk && cw::starts_non_ideographic_text_presentation_seq(c) =>
                    {
                        1
                    }

                    Some(VariationSelector::Vs16) if cw::starts_emoji_presentation_seq(c) => 2,
                    _ => cw::width(c, is_cjk).unwrap_or(0),
                };
                (sum + add, None)
            }
        })
        .0
}
