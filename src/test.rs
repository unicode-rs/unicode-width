// Copyright 2012-2025 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::lookup::*;
use crate::width_info::WidthInfo;

#[path = "gen/tables_test.rs"]
mod tables_test;

use tables_test::*;

fn str_width_test(s: &str, init: WidthInfo) -> (isize, WidthInfo) {
    s.chars()
        .rfold((0, init), |(sum, next_info), c| -> (isize, WidthInfo) {
            let (add, info) = width_in_str(c, next_info);
            (sum.checked_add(isize::from(add)).unwrap(), info)
        })
}

#[cfg(feature = "cjk")]
fn str_width_test_cjk(s: &str, init: WidthInfo) -> (isize, WidthInfo) {
    s.chars()
        .rfold((0, init), |(sum, next_info), c| -> (isize, WidthInfo) {
            let (add, info) = width_in_str_cjk(c, next_info);
            (sum.checked_add(isize::from(add)).unwrap(), info)
        })
}

#[test]
fn test_variation_selector_fallback() {
    for (s, narrow, _cjk) in [
        ("\u{FE0F}#", 1, 1),
        ("\r\u{FE0F}#", 2, 2),
        ("#\u{0301}\u{FE0F}", 1, 1),
        ("#\u{FE00}\u{FE0F}", 1, 1),
        ("#\u{FE0F}\u{FE0F}", 2, 2),
        ("\u{2018}\u{0301}\u{FE01}", 1, 2),
        ("\u{180B}\u{180F}\u{E0100}\u{E01EF}", 0, 0),
    ] {
        assert_eq!(str_width(s.chars()), narrow, "{s:?}");
        #[cfg(feature = "cjk")]
        assert_eq!(str_width_cjk(s.chars()), _cjk, "{s:?}");
    }
}

#[test]
fn test_emoji_qualification_policy() {
    for (s, narrow, _cjk) in [
        ("\u{1F469}\u{200D}\u{2696}", 3, 3),
        ("\u{2764}\u{200D}\u{1F525}", 3, 3),
        ("\u{1F469}\u{FE0E}\u{200D}\u{1F680}", 4, 4),
        ("\u{2648}\u{FE0E}\u{200D}\u{1F680}", 3, 4),
        ("\u{1F469}\u{200D}\u{2696}\u{FE0E}", 3, 3),
    ] {
        assert_eq!(str_width(s.chars()), narrow, "{s:?}");
        #[cfg(feature = "cjk")]
        assert_eq!(str_width_cjk(s.chars()), _cjk, "{s:?}");
    }
}

#[test]
fn test_normalization() {
    for &(orig, nfc, nfd, nfkc, nfkd) in &NORMALIZATION_TEST {
        for init in NORMALIZATION_TEST_WIDTHS {
            assert_eq!(
                str_width_test(orig, init),
                str_width_test(nfc, init),
                "width of X = {orig:?} differs from toNFC(X) = {nfc:?} with mode {init:X?}",
            );
            assert_eq!(
                str_width_test(orig, init),
                str_width_test(nfd, init),
                "width of X = {orig:?} differs from toNFD(X) = {nfd:?} with mode {init:X?}",
            );
            assert_eq!(
                str_width_test(nfkc, init),
                str_width_test(nfkd, init),
                "width of toNFKC(X) = {nfkc:?} differs from toNFKD(X) = {nfkd:?} with mode {init:X?}",
            );
        }

        #[cfg(feature = "cjk")]
        for init in NORMALIZATION_TEST_WIDTHS_CJK {
            assert_eq!(
                str_width_test_cjk(orig, init),
                str_width_test_cjk(nfc, init),
                "CJK width of X = {orig:?} differs from toNFC(X) = {nfc:?} with mode {init:X?}",
            );
            assert_eq!(
                str_width_test_cjk(orig, init),
                str_width_test_cjk(nfd, init),
                "CJK width of X = {orig:?} differs from toNFD(X) = {nfd:?} with mode {init:X?}",
            );
            assert_eq!(
                str_width_test_cjk(nfkc, init),
                str_width_test_cjk(nfkd, init),
                "CJK width of toNFKC(X) = {nfkc:?} differs from toNFKD(X) = {nfkd:?} with mode {init:?}",
            );
        }
    }
}
