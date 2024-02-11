# unicode-width

Determine displayed width of `char` and `str` types according to
[Unicode Standard Annex #11][UAX11] rules.

[UAX11]: http://www.unicode.org/reports/tr11/

[![Build Status](https://travis-ci.org/unicode-rs/unicode-width.svg)](https://travis-ci.org/unicode-rs/unicode-width)

[Documentation](https://unicode-rs.github.io/unicode-width/unicode_width/index.html)

```rust
extern crate unicode_width;

use unicode_width::UnicodeWidthStr;

fn main() {
    let teststr = "Ｈｅｌｌｏ, ｗｏｒｌｄ!";
    let width = UnicodeWidthStr::width(teststr);
    println!("{}", teststr);
    println!("The above string is {} columns wide.", width);
    let width = teststr.width_cjk();
    println!("The above string is {} columns wide (CJK).", width);
}
```

**NOTE:** The computed width values may not match the actual rendered column
width. For example, the woman scientist emoji comprises of a woman emoji, a
zero-width joiner and a microscope emoji. Such [emoji ZWJ sequences](https://www.unicode.org/reports/tr51/#Emoji_ZWJ_Sequences)
are considered to have the sum of the widths of their constituent parts:

```rust
extern crate unicode_width;
use unicode_width::UnicodeWidthStr;

fn main() {
    assert_eq!(UnicodeWidthStr::width("👩"), 2); // Woman
    assert_eq!(UnicodeWidthStr::width("🔬"), 2); // Microscope
    assert_eq!(UnicodeWidthStr::width("👩‍🔬"), 4); // Woman scientist
}
```

Additionally, [defective combining character sequences](https://unicode.org/glossary/#defective_combining_character_sequence)
and nonstandard [Korean jamo](https://unicode.org/glossary/#jamo) sequences may
be rendered with a different width than what this crate says. (This is not an
exhaustive list.)

## features

unicode-width does not depend on libstd, so it can be used in crates
with the `#![no_std]` attribute.

## crates.io

You can use this package in your project by adding the following
to your `Cargo.toml`:

```toml
[dependencies]
unicode-width = "0.1.7"
```
