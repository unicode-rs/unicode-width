// Copyright 2012-2026 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[path = "gen/lookup.rs"]
mod gen;

pub use gen::*;

use crate::width_info::WidthInfo;

#[inline]
pub fn str_width<S: DoubleEndedIterator<Item = char>>(s: S) -> usize {
    s.rfold(
        (0, WidthInfo::DEFAULT),
        |(sum, next_info), c| -> (usize, WidthInfo) {
            let (add, info) = width_in_str(c, next_info);
            (sum.wrapping_add_signed(isize::from(add)), info)
        },
    )
    .0
}

#[cfg(feature = "cjk")]
#[inline]
pub fn str_width_cjk<S: DoubleEndedIterator<Item = char>>(s: S) -> usize {
    s.rfold(
        (0, WidthInfo::DEFAULT),
        |(sum, next_info), c| -> (usize, WidthInfo) {
            let (add, info) = width_in_str_cjk(c, next_info);
            (sum.wrapping_add_signed(isize::from(add)), info)
        },
    )
    .0
}
