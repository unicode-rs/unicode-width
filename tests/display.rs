#![cfg(feature = "display")]

use unicode_width::UnicodeWidthStr;

#[test]
fn basic() {
    for s in [
        "",
        "\0",
        "a",
        "abc",
        "¡Olé!",
        "kilimanjaro",
        "Κύριε, ἐλέησον",
    ] {
        assert_eq!(format!("{}", s.using_width()), s);
    }
}

#[test]
fn basic_with_args() {
    for min_width in 0..20 {
        for max_width in 0..20 {
            for s in [
                "",
                "\0",
                "a",
                "abc",
                "¡Olé!",
                "kilimanjaro",
                "Κύριε, ἐλέησον",
            ] {
                assert_eq!(
                    format!(
                        "{:a<min$.max$}",
                        s.using_width(),
                        min = min_width,
                        max = max_width
                    ),
                    format!("{:a<min$.max$}", s, min = min_width, max = max_width)
                );

                assert_eq!(
                    format!(
                        "{:a^min$.max$}",
                        s.using_width(),
                        min = min_width,
                        max = max_width
                    ),
                    format!("{:a^min$.max$}", s, min = min_width, max = max_width)
                );

                assert_eq!(
                    format!(
                        "{:a>min$.max$}",
                        s.using_width(),
                        min = min_width,
                        max = max_width
                    ),
                    format!("{:a>min$.max$}", s, min = min_width, max = max_width)
                );
            }
        }
    }
}

#[test]
fn trunc() {
    let smol_str = "汉字".using_width();
    let smol_prefixes = ["", "", "汉", "汉", "汉字", "汉字"];
    for (width, prefix) in smol_prefixes.into_iter().enumerate() {
        assert_eq!(format!("{smol_str:.width$}"), prefix, "width: {width}");
    }

    let med_str = "a汉字b".using_width();
    let med_prefixes = ["", "a", "a", "a汉", "a汉", "a汉字", "a汉字b", "a汉字b"];
    for (width, prefix) in med_prefixes.into_iter().enumerate() {
        assert_eq!(format!("{med_str:.width$}"), prefix, "width: {width}");
    }

    let huge_str =
        "\u{200B}\u{200E}a\u{0301}汉字\r\nb\u{2764}\u{FE0F}c\u{2648}\u{FE0E}\u{FF9E}".using_width();
    let huge_prefixes = [
        "\u{200B}\u{200E}",
        "\u{200B}\u{200E}a\u{0301}",
        "\u{200B}\u{200E}a\u{0301}",
        "\u{200B}\u{200E}a\u{0301}汉",
        "\u{200B}\u{200E}a\u{0301}汉",
        "\u{200B}\u{200E}a\u{0301}汉字",
        "\u{200B}\u{200E}a\u{0301}汉字\r\n",
        "\u{200B}\u{200E}a\u{0301}汉字\r\nb",
        "\u{200B}\u{200E}a\u{0301}汉字\r\nb",
        "\u{200B}\u{200E}a\u{0301}汉字\r\nb\u{2764}\u{FE0F}",
        "\u{200B}\u{200E}a\u{0301}汉字\r\nb\u{2764}\u{FE0F}c",
        "\u{200B}\u{200E}a\u{0301}汉字\r\nb\u{2764}\u{FE0F}c\u{2648}\u{FE0E}\u{FF9E}",
    ];

    for (width, prefix) in huge_prefixes.into_iter().enumerate() {
        assert_eq!(format!("{huge_str:.width$}"), prefix, "width: {width}");
    }
}

#[test]
fn pad() {
    let string = "\u{2764}\u{FE0F}a".using_width();

    assert_eq!(format!("{string:q<7}"), "\u{2764}\u{FE0F}aqqqq");
    assert_eq!(format!("{string:q^7}"), "qq\u{2764}\u{FE0F}aqq");
    assert_eq!(format!("{string:q>7}"), "qqqq\u{2764}\u{FE0F}a");

    assert_eq!(format!("{string:字<7}"), "\u{2764}\u{FE0F}a字字");
    assert_eq!(format!("{string:字^7}"), "字\u{2764}\u{FE0F}a字");
    assert_eq!(format!("{string:字>7}"), "字字\u{2764}\u{FE0F}a");

    assert_eq!(format!("{string:\u{0301}<7}"), "\u{2764}\u{FE0F}a    ");
    assert_eq!(format!("{string:\u{0301}^7}"), "  \u{2764}\u{FE0F}a  ");
    assert_eq!(format!("{string:\u{0301}>7}"), "    \u{2764}\u{FE0F}a");

    assert_eq!(format!("{string:q<8}"), "\u{2764}\u{FE0F}aqqqqq");
    assert_eq!(format!("{string:q^8}"), "qq\u{2764}\u{FE0F}aqqq");
    assert_eq!(format!("{string:q>8}"), "qqqqq\u{2764}\u{FE0F}a");

    assert_eq!(format!("{string:字<8}"), "\u{2764}\u{FE0F}a 字字");
    assert_eq!(format!("{string:字^8}"), "字\u{2764}\u{FE0F}a 字");
    assert_eq!(format!("{string:字>8}"), "字字 \u{2764}\u{FE0F}a");

    assert_eq!(format!("{string:\u{0301}<8}"), "\u{2764}\u{FE0F}a     ");
    assert_eq!(format!("{string:\u{0301}^8}"), "  \u{2764}\u{FE0F}a   ");
    assert_eq!(format!("{string:\u{0301}>8}"), "     \u{2764}\u{FE0F}a");

    let string = "a".using_width();
    assert_eq!(format!("{string:字^7}"), "字a字字");

    let string = "字".using_width();
    assert_eq!(format!("{string:<3}"), "字 ");
    assert_eq!(format!("{string:^3}"), "字 ");
    assert_eq!(format!("{string:>3}"), " 字");
    assert_eq!(format!("{string:<4}"), "字  ");
    assert_eq!(format!("{string:^4}"), " 字 ");
    assert_eq!(format!("{string:>4}"), "  字");
}
