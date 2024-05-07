// Copyright 2012-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

#[test]
fn test_str() {
    assert_eq!(UnicodeWidthStr::width("ｈｅｌｌｏ"), 10);
    assert_eq!("ｈｅｌｌｏ".width_cjk(), 10);
    assert_eq!(UnicodeWidthStr::width("\0\0\0\x01\x01"), 0);
    assert_eq!("\0\0\0\x01\x01".width_cjk(), 0);
    assert_eq!(UnicodeWidthStr::width(""), 0);
    assert_eq!("".width_cjk(), 0);
    assert_eq!(
        UnicodeWidthStr::width("\u{2081}\u{2082}\u{2083}\u{2084}"),
        4
    );
    assert_eq!("\u{2081}\u{2082}\u{2083}\u{2084}".width_cjk(), 8);
}

#[test]
fn test_emoji() {
    // Example from the README.
    assert_eq!(UnicodeWidthStr::width("👩"), 2); // Woman
    assert_eq!(UnicodeWidthStr::width("🔬"), 2); // Microscope
    assert_eq!(UnicodeWidthStr::width("👩‍🔬"), 4); // Woman scientist
}

#[test]
fn test_char() {
    assert_eq!(UnicodeWidthChar::width('ｈ'), Some(2));
    assert_eq!('ｈ'.width_cjk(), Some(2));
    assert_eq!(UnicodeWidthChar::width('\x00'), Some(0));
    assert_eq!('\x00'.width_cjk(), Some(0));
    assert_eq!(UnicodeWidthChar::width('\x01'), None);
    assert_eq!('\x01'.width_cjk(), None);
    assert_eq!(UnicodeWidthChar::width('\u{2081}'), Some(1));
    assert_eq!('\u{2081}'.width_cjk(), Some(2));
}

#[test]
fn test_char2() {
    assert_eq!(UnicodeWidthChar::width('\x00'), Some(0));
    assert_eq!('\x00'.width_cjk(), Some(0));

    assert_eq!(UnicodeWidthChar::width('\x0A'), None);
    assert_eq!('\x0A'.width_cjk(), None);

    assert_eq!(UnicodeWidthChar::width('w'), Some(1));
    assert_eq!('w'.width_cjk(), Some(1));

    assert_eq!(UnicodeWidthChar::width('ｈ'), Some(2));
    assert_eq!('ｈ'.width_cjk(), Some(2));

    assert_eq!(UnicodeWidthChar::width('\u{AD}'), Some(0));
    assert_eq!('\u{AD}'.width_cjk(), Some(0));

    assert_eq!(UnicodeWidthChar::width('\u{1160}'), Some(0));
    assert_eq!('\u{1160}'.width_cjk(), Some(0));

    assert_eq!(UnicodeWidthChar::width('\u{a1}'), Some(1));
    assert_eq!('\u{a1}'.width_cjk(), Some(2));

    assert_eq!(UnicodeWidthChar::width('\u{300}'), Some(0));
    assert_eq!('\u{300}'.width_cjk(), Some(0));
}

#[test]
fn unicode_12() {
    assert_eq!(UnicodeWidthChar::width('\u{1F971}'), Some(2));
}

#[test]
fn test_default_ignorable() {
    assert_eq!(UnicodeWidthChar::width('\u{E0000}'), Some(0));

    assert_eq!(UnicodeWidthChar::width('\u{1160}'), Some(0));
    assert_eq!(UnicodeWidthChar::width('\u{3164}'), Some(0));
    assert_eq!(UnicodeWidthChar::width('\u{FFA0}'), Some(0));
}

#[test]
fn test_jamo() {
    assert_eq!(UnicodeWidthChar::width('\u{1100}'), Some(2));
    assert_eq!(UnicodeWidthChar::width('\u{A97C}'), Some(2));
    // Special case: U+115F HANGUL CHOSEONG FILLER
    assert_eq!(UnicodeWidthChar::width('\u{115F}'), Some(2));
    assert_eq!(UnicodeWidthChar::width('\u{1160}'), Some(0));
    assert_eq!(UnicodeWidthChar::width('\u{D7C6}'), Some(0));
    assert_eq!(UnicodeWidthChar::width('\u{11A8}'), Some(0));
    assert_eq!(UnicodeWidthChar::width('\u{D7FB}'), Some(0));
}

#[test]
fn test_prepended_concatenation_marks() {
    assert_eq!(UnicodeWidthChar::width('\u{0600}'), Some(1));
    assert_eq!(UnicodeWidthChar::width('\u{070F}'), Some(1));
    assert_eq!(UnicodeWidthChar::width('\u{08E2}'), Some(1));
    assert_eq!(UnicodeWidthChar::width('\u{110BD}'), Some(1));
}

#[test]
fn test_interlinear_annotation_chars() {
    assert_eq!(UnicodeWidthChar::width('\u{FFF9}'), Some(1));
    assert_eq!(UnicodeWidthChar::width('\u{FFFA}'), Some(1));
    assert_eq!(UnicodeWidthChar::width('\u{FFFB}'), Some(1));
}

#[test]
fn test_hieroglyph_format_controls() {
    assert_eq!(UnicodeWidthChar::width('\u{13430}'), Some(1));
    assert_eq!(UnicodeWidthChar::width('\u{13436}'), Some(1));
    assert_eq!(UnicodeWidthChar::width('\u{1343C}'), Some(1));
}

#[test]
fn test_marks() {
    // Nonspacing marks have 0 width
    assert_eq!(UnicodeWidthChar::width('\u{0301}'), Some(0));
    // Enclosing marks have 0 width
    assert_eq!(UnicodeWidthChar::width('\u{20DD}'), Some(0));
    // Some spacing marks have width 1
    assert_eq!(UnicodeWidthChar::width('\u{09CB}'), Some(1));
    // But others have width 0
    assert_eq!(UnicodeWidthChar::width('\u{09BE}'), Some(0));
}

#[test]
fn test_canonical_equivalence() {
    let norm_file = BufReader::new(
        File::open("tests/NormalizationTest.txt")
            .expect("run `unicode.py` first to download `NormalizationTest.txt`"),
    );
    for line in norm_file.lines() {
        let line = line.unwrap();
        if line.is_empty() || line.starts_with('#') || line.starts_with('@') {
            continue;
        }

        let mut forms_iter = line.split(';').map(|substr| -> String {
            substr
                .split(' ')
                .map(|s| char::try_from(u32::from_str_radix(s, 16).unwrap()).unwrap())
                .collect()
        });

        let orig = forms_iter.next().unwrap();
        let nfc = forms_iter.next().unwrap();
        let nfd = forms_iter.next().unwrap();
        let nfkc = forms_iter.next().unwrap();
        let nfkd = forms_iter.next().unwrap();

        assert_eq!(
            orig.width(),
            nfc.width(),
            "width of X == {orig:?} differs from toNFC(X) == {nfc:?}"
        );

        assert_eq!(
            orig.width(),
            nfd.width(),
            "width of X == {orig:?} differs from toNFD(X) == {nfd:?}"
        );

        assert_eq!(
            nfkc.width(),
            nfkd.width(),
            "width of toNFKC(X) == {nfkc:?} differs from toNFKD(X) == {nfkd:?}"
        );
    }
}

#[test]
fn test_emoji_presentation() {
    assert_eq!(UnicodeWidthChar::width('\u{0023}'), Some(1));
    assert_eq!(UnicodeWidthChar::width('\u{FE0F}'), Some(0));
    assert_eq!(UnicodeWidthStr::width("\u{0023}\u{FE0F}"), 2);
    assert_eq!(UnicodeWidthStr::width("a\u{0023}\u{FE0F}a"), 4);
    assert_eq!(UnicodeWidthStr::width("\u{0023}a\u{FE0F}"), 2);
    assert_eq!(UnicodeWidthStr::width("a\u{FE0F}"), 1);
    assert_eq!(UnicodeWidthStr::width("\u{0023}\u{0023}\u{FE0F}a"), 4);

    assert_eq!(UnicodeWidthStr::width("\u{002A}\u{FE0F}"), 2);
    assert_eq!(UnicodeWidthStr::width("\u{23F9}\u{FE0F}"), 2);
    assert_eq!(UnicodeWidthStr::width("\u{24C2}\u{FE0F}"), 2);
    assert_eq!(UnicodeWidthStr::width("\u{1F6F3}\u{FE0F}"), 2);
    assert_eq!(UnicodeWidthStr::width("\u{1F700}\u{FE0F}"), 1);
}

#[test]
fn test_text_presentation() {
    assert_eq!('\u{FE0E}'.width(), Some(0));

    assert_eq!('\u{2648}'.width(), Some(2));
    assert_eq!("\u{2648}\u{FE0E}".width(), 1);
    assert_eq!("\u{2648}\u{FE0E}".width_cjk(), 2);

    assert_eq!("\u{1F21A}\u{FE0E}".width(), 2);
    assert_eq!("\u{1F21A}\u{FE0E}".width_cjk(), 2);

    assert_eq!("\u{0301}\u{FE0E}".width(), 0);
    assert_eq!("\u{0301}\u{FE0E}".width_cjk(), 0);

    assert_eq!("a\u{FE0E}".width(), 1);
    assert_eq!("a\u{FE0E}".width_cjk(), 1);

    assert_eq!("𘀀\u{FE0E}".width(), 2);
    assert_eq!("𘀀\u{FE0E}".width_cjk(), 2);
}
