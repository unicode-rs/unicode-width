import io
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

import unicode


class UnicodeDataTests(unittest.TestCase):
    def test_property_records(self):
        records = " # comment\n0041;AI\n 0042..0043 ; AL#comment\n0044;AIX\n0045;AI"
        with patch.object(unicode, "fetch_open", return_value=io.StringIO(records)):
            found = []
            unicode.load_property("LineBreak.txt", "AI|AL", found.append)
        self.assertEqual(found, [0x41, 0x42, 0x43, 0x45])

    def test_east_asian_width_records_and_defaults(self):
        records = """# @missing: 0000..10FFFF; N
003D;A
0041;Na
00A1;A#arbitrary comment
00A2;H
00A3;F
00A4..00A5;W
00A6;N
2260..2261;N
F900..FAFF;W
20000..2FFFD;W
30000..3FFFD;W
# @missing: 0041..0043; W
# @missing: 0000..10FFFF; N
# @missing: 0042; W
"""
        with patch.object(
            unicode, "fetch_open",
            side_effect=lambda name: io.StringIO({
                "EastAsianWidth.txt": records,
                "UnicodeData.txt": "2260;NOT EQUAL TO;Sm;0;ON;003D 0338;\n2261;COMPATIBILITY FORM;Sm;0;ON;<compat> 003D 0338;\n",
            }.get(name, "")),
        ):
            widths = unicode.load_east_asian_widths()
        narrow, wide, ambiguous = unicode.EastAsianWidth
        for cp, expected in [
            (0, narrow), (0x41, narrow), (0x42, wide), (0x43, narrow),
            (0xA1, ambiguous), (0xA2, narrow), (0xA3, wide), (0xA4, wide),
            (0xA5, wide), (0xA6, narrow), (0xA7, narrow),
            (0x2260, ambiguous), (0x2261, narrow),
            (0xF8FF, narrow), (0xF900, wide), (0xFAFF, wide), (0xFB00, narrow),
            (0x1FFFF, narrow), (0x20000, wide), (0x2FFFD, wide), (0x2FFFE, narrow),
            (0x30000, wide), (0x3FFFD, wide), (0x3FFFE, narrow), (0x10FFFF, narrow),
        ]:
            with self.subTest(cp=hex(cp)):
                self.assertEqual(widths[cp], expected)

    def test_solidus_canonical_decompositions(self):
        records = """0342;RECURSIVE;Mn;0;NSM;0341;
0341;CANONICAL;Mn;0;NSM;0301 0301;
0343;COMPATIBILITY;Mn;0;NSM;<compat> 0301;
0344;EMPTY;Mn;0;NSM;;
0345;MIXED;Mn;0;NSM;0301 0041;
0346;NUMERAL;No;0;L;;;;300;N;;;;;
"""
        with patch.object(unicode, "fetch_open", side_effect=lambda name: io.StringIO(
            records if name == "UnicodeData.txt" else "0300..0301;230\n"
        )):
            widths = [unicode.WidthState.NARROW] * 0x347
            self.assertEqual(unicode.load_solidus_transparent([], widths), [(0x300, 0x301), (0x341, 0x342)])

    def test_versioned_cache(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "UnicodeData.txt").write_text("stale data", encoding="utf-8")
            expected = root / f"{unicode.UNICODE_VERSION}-UnicodeData.txt"

            def download(url, path):
                self.assertEqual(url, f"https://www.unicode.org/Public/{unicode.UNICODE_VERSION}/ucd/UnicodeData.txt")
                self.assertEqual(Path(path), expected)
                expected.write_text("current data", encoding="utf-8")

            with patch.object(unicode.urllib.request, "urlretrieve", side_effect=download) as fetch:
                for _ in range(2):
                    with unicode.fetch_open("UnicodeData.txt", directory) as data:
                        self.assertEqual(data.read(), "current data")
                fetch.assert_called_once()

    def test_version_headers(self):
        version = unicode.UNICODE_VERSION
        emoji_version = version.rsplit(".", 1)[0]
        for filename, emoji, header in [
            ("extracted/DerivedGeneralCategory.txt", False, f"# DerivedGeneralCategory-{version}.txt\n"),
            ("ReadMe.txt", False, f"Final data files for version {version} of the\nUnicode Character Database.\n"),
            ("emoji/emoji-data.txt", False, f"# emoji-data.txt\n# Version: {version}\n"),
            ("emoji-test.txt", True, f"# emoji-test.txt\n# Version: {emoji_version}\n"),
        ]:
            with self.subTest(filename=filename), tempfile.TemporaryDirectory() as directory:
                basename = Path(filename).name
                cached = basename if emoji else f"{version}-{basename}"
                path = Path(directory) / cached
                path.write_text(header, encoding="utf-8")
                with patch.object(unicode.urllib.request, "urlretrieve") as fetch:
                    with unicode.fetch_open(filename, directory, emoji) as data:
                        self.assertEqual(data.read(), header)
                    fetch.assert_not_called()
                for invalid in [header.replace(version, "0.0.0").replace(emoji_version, "0.0"), "no version\n"]:
                    path.write_text(invalid, encoding="utf-8")
                    with self.assertRaisesRegex(ValueError, "expected Unicode"):
                        unicode.fetch_open(filename, directory, emoji)


if __name__ == "__main__":
    unittest.main()
