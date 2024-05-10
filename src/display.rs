use core::fmt::{self, Write};

use unicode_segmentation::UnicodeSegmentation;

use crate::{UnicodeWidthChar, UnicodeWidthStr};

/// A wrapper around a [`str`] with a [`fmt::Display`] impl
/// that performs padding, truncation, and alignment based on
/// the string width according to this crate (non-CJK).
///
/// Produced via [`UnicodeWidthStr::using_width`];
/// see its documentation for more.
#[derive(PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct StrWithWidth(str);

impl StrWithWidth {
    /// The advance width of the `string`
    /// (equivalent to [`UnicodeWidthStr::width`]).
    #[inline]
    pub fn width(&self) -> usize {
        self.0.width()
    }
}

impl PartialEq<str> for StrWithWidth {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        &self.0 == other
    }
}

impl AsRef<str> for StrWithWidth {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsMut<str> for StrWithWidth {
    #[inline]
    fn as_mut(&mut self) -> &mut str {
        &mut self.0
    }
}

impl AsRef<StrWithWidth> for str {
    #[inline]
    fn as_ref(&self) -> &StrWithWidth {
        // SAFETY: `repr(transparent)` ensures compatible types
        unsafe { core::mem::transmute(self) }
    }
}

impl AsMut<StrWithWidth> for str {
    #[inline]
    fn as_mut(&mut self) -> &mut StrWithWidth {
        // SAFETY: `repr(transparent)` ensures compatible types
        unsafe { core::mem::transmute(self) }
    }
}

impl fmt::Display for StrWithWidth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Fast path
        if f.width().is_none() && f.precision().is_none() {
            return f.write_str(&self.0);
        }

        // Truncate the string to maximum width
        let (truncated, truncated_width) = if let Some(max_width) = f.precision() {
            let mut truncated_width: usize = 0;
            let mut truncated = &self.0;
            for (seg_offset, seg) in self.0.grapheme_indices(true) {
                let new_width = truncated_width + seg.width();
                if new_width > max_width {
                    truncated = &self.0[..seg_offset];
                    break;
                } else {
                    truncated_width = new_width;
                }
            }
            (truncated, truncated_width)
        } else {
            (&self.0, self.0.width())
        };

        // Pad the string to minimum width
        if let Some(padding) = f
            .width()
            .and_then(|min_width| min_width.checked_sub(truncated_width))
            .filter(|&padding| padding > 0)
        {
            let align = f.align().unwrap_or(fmt::Alignment::Left);

            let mut fill_char = f.fill();
            let mut fill_char_width = fill_char.width().unwrap_or(1);

            // If we try to fill with a zero-sized char, we'll never succeed, so fall back to space
            if fill_char_width == 0 {
                fill_char = ' ';
                fill_char_width = 1;
            }

            let (pre_pre_pad, pre_pad, post_pad, post_post_pad) = match align {
                fmt::Alignment::Left => {
                    (0, 0, padding % fill_char_width, padding / fill_char_width)
                }
                fmt::Alignment::Right => {
                    (padding / fill_char_width, padding % fill_char_width, 0, 0)
                }
                fmt::Alignment::Center => {
                    let (left_padding, right_padding) = (padding / 2, (padding + 1) / 2);
                    let (pre_pre_pad, mut pre_pad, mut post_pad, mut post_post_pad) = {
                        (
                            left_padding / fill_char_width,
                            left_padding % fill_char_width,
                            right_padding % fill_char_width,
                            right_padding / fill_char_width,
                        )
                    };
                    if let Some(diff) = pre_pad.checked_sub(fill_char_width - post_pad) {
                        pre_pad = 0;
                        post_pad = diff;
                        post_post_pad += 1;
                    }
                    (pre_pre_pad, pre_pad, post_pad, post_post_pad)
                }
            };

            for _ in 0..pre_pre_pad {
                f.write_char(fill_char)?;
            }
            for _ in 0..pre_pad {
                f.write_char(' ')?;
            }
            f.write_str(truncated)?;
            for _ in 0..post_pad {
                f.write_char(' ')?;
            }
            for _ in 0..post_post_pad {
                f.write_char(fill_char)?;
            }

            Ok(())
        } else {
            f.write_str(truncated)
        }
    }
}

impl fmt::Debug for StrWithWidth {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}
