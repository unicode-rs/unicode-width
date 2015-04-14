# unicode-width

Determine displayed width of `char` and `str` types according to
[Unicode Standard Annex #11](http://www.unicode.org/reports/tr11/)
rules.

[![Build Status](https://travis-ci.org/unicode-rs/unicode-width.svg?branch=master)](https://travis-ci.org/unicode-rs/unicode-width)

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

[Documentation](http://unicode-rs.github.io/unicode-width/unicode_width/)
