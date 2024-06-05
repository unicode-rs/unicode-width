# Changelog

All notable changes to this project will be documented in this file.

## [0.1.13] - 2024-06-04

- [eec13fa2](https://github.com/unicode-rs/unicode-width/commit/eec13fa271c880198d55a6c760f27b710b3aedce): Add more canonical equivalence tests

  Test that all canonically equivalent sequences
  in Unicode's `NormalizationTest.txt` have the same width.
  Currently no changes need to be made to the width logic
  to ensure these tests pass. However, Unicode 16
  is adding a few new characters that will be problematic
  (the Kirat Rai vowel signs:
  <https://www.unicode.org/charts/PDF/Unicode-16.0/U160-16D40.pdf>).
  Adding this test in advance ensures that we won't forget
  to account for these changes when the time comes.
- [d7755f22](https://github.com/unicode-rs/unicode-width/commit/d7755f22445c81d6d00ce1d963bb51a282f23e03): Remove old normalization test

  The new test subsumes it
- [81ae8e2c](https://github.com/unicode-rs/unicode-width/commit/81ae8e2cf03f41559f3cd520339a1df225c59be2): Specify unicode version in unicode.py

  Ensures CI will keep passing until we choose to update
- [dbaa10eb](https://github.com/unicode-rs/unicode-width/commit/dbaa10eb8ffc99129fd34084b7be105e122a5b41): Check `NormalizationTest.txt` into CI
- [cf658288](https://github.com/unicode-rs/unicode-width/commit/cf65828885ac31d0e47cd084b9cbdcd7716fe635): Even more tests
- [fdd6f397](https://github.com/unicode-rs/unicode-width/commit/fdd6f397dcbd96a9fe229f8443a4ec4c8f4991b5): Diff `NormalizationTest.txt` in CI
- [acd8db94](https://github.com/unicode-rs/unicode-width/commit/acd8db94ca80bc250dc063d40a79da9a2a4cf861): Fix CI
- [decf378e](https://github.com/unicode-rs/unicode-width/commit/decf378e7b8030ed0e4466baf71cfe787189737b): More CI fix
- [74c83942](https://github.com/unicode-rs/unicode-width/commit/74c8394211ea01c88b5866bc42b948e2b0baa3e6): Merge pull request #42 from Jules-Bertholet/more-normalization-tests

  Add more canonical equivalence tests
- [2e2d3bb1](https://github.com/unicode-rs/unicode-width/commit/2e2d3bb17d6ec35e13bcb05baa9587e06cb5941c): Support text presentation sequences
- [3aa94a5b](https://github.com/unicode-rs/unicode-width/commit/3aa94a5b1621a1c122d6fba48e8878799d24a4f3): Merge pull request #43 from Jules-Bertholet/text-presentation

  Support text presentation sequences
- [05ee35d3](https://github.com/unicode-rs/unicode-width/commit/05ee35d37b3ff04a7ae71c838b8a6d2c0b78fe80): Remove soft hyphen special case
- [86970a10](https://github.com/unicode-rs/unicode-width/commit/86970a10ea750e6172f1227edf227aae6d6ff625): Merge pull request #44 from Jules-Bertholet/dont-be-shy

  Remove soft hyphen special case
- [4efb1803](https://github.com/unicode-rs/unicode-width/commit/4efb1803faa054f1bea3c0457275ad3c8610170b): Control characters have width 1
- [3063422f](https://github.com/unicode-rs/unicode-width/commit/3063422f38039be82db2b3bd331c8a9f42d57ef1): Merge pull request #45 from Jules-Bertholet/control

  Assign width 1 to control characters
- [a7a10568](https://github.com/unicode-rs/unicode-width/commit/a7a1056858fe3242e6df8e1b86670b840648a02b): Test width = sum(grapheme cluster widths)
- [6edfc609](https://github.com/unicode-rs/unicode-width/commit/6edfc609fb538d108c396a9643da99f4b84db533): Merge pull request #46 from Jules-Bertholet/test-grapheme-clusters

  Test width = sum(grapheme cluster widths)
- [ded852c2](https://github.com/unicode-rs/unicode-width/commit/ded852c29cd846474170bdcfa6546a3ca3ad4ffc): Revert "Test width = sum(grapheme cluster widths)"

  This reverts commit a7a1056858fe3242e6df8e1b86670b840648a02b.
- [b3ab6332](https://github.com/unicode-rs/unicode-width/commit/b3ab633204757e8c670e7eb04a1755ab64e80a0c): Support Lisu tone letters
- [7cb4f390](https://github.com/unicode-rs/unicode-width/commit/7cb4f390756f7427fa75484566776c274e294b63): Merge pull request #48 from Jules-Bertholet/lisu-tone

  Lisu tone letters
- [d6c1554b](https://github.com/unicode-rs/unicode-width/commit/d6c1554bc1b293706ec70327a8d5cd2a3e41bdc7): Fix CI link in README
- [47bac329](https://github.com/unicode-rs/unicode-width/commit/47bac329d88a3a59bfe93b2b1ae2212caf07f52f): Merge pull request #50 from Jules-Bertholet/remove-old-ci

  Fix CI link in README
- [934c875b](https://github.com/unicode-rs/unicode-width/commit/934c875bff19519260ab29fda0ee50a21b52bd47): Mark U+070F SYRIAC ABBREVIATION MARK as zero width
- [3b56f6dd](https://github.com/unicode-rs/unicode-width/commit/3b56f6ddcb379b8f16ac34f52f415a68d604420a): Mark U+A8FA DEVANAGARI CARET as zero-width
- [3742586f](https://github.com/unicode-rs/unicode-width/commit/3742586f8411102a3293c774f9fda6571e37534c): Mark more `Prepended_Concatenation_Mark`s as non-advancing
- [da626eff](https://github.com/unicode-rs/unicode-width/commit/da626eff30ce0f5be074fe71c8d81756e0a7701a): Merge pull request #49 from Jules-Bertholet/syriac-abbreviation-mark

  Mark U+070F and U+A8FA as zero width
- [a2db56bb](https://github.com/unicode-rs/unicode-width/commit/a2db56bbd0566e6466829427fd0c38929e6f20c6): Refactor `unicode.py`

  - Align tables
  - Use helper function to parse properties
- [dc86c748](https://github.com/unicode-rs/unicode-width/commit/dc86c7485fba9f084d9adcaaad86a5c9e4470a99): Assign the same CJK width to canonically equivalent strings
- [d00d3571](https://github.com/unicode-rs/unicode-width/commit/d00d35711f438b1222ffcbeae8d420ca9617d38f): Merge pull request #52 from Jules-Bertholet/canonically-equivalent-eaw

  Assign the same CJK width to canonically equivalent strings
- [3b82122a](https://github.com/unicode-rs/unicode-width/commit/3b82122acd576e800378f66e5bfeaf04d805523e): Adapt for rustc-dep-of-std build
- [e370cb8c](https://github.com/unicode-rs/unicode-width/commit/e370cb8c4a734287019e76045fe82ae900517b7a): Merge pull request #54 from krasimirgg/rustc

  adapt for rustc-dep-of-std build
- [612877a5](https://github.com/unicode-rs/unicode-width/commit/612877a5d70bfc7c93b4bc70b740aa292dd4f48a): Bump to 0.1.13
## [0.1.12] - 2024-04-26

- [aed33e98](https://github.com/unicode-rs/unicode-width/commit/aed33e984a3cf726a5015c61153d85d33e02c4a1): Treat `Default_Ignorable_Code_Point`s as zero-width
- [397ab07c](https://github.com/unicode-rs/unicode-width/commit/397ab07cd7a10ee00ad2f9f57dd768e6ba42b088): Treat all jungseong and jongseong jamo as 0-width

  Fixes #26
- [a6b5a52d](https://github.com/unicode-rs/unicode-width/commit/a6b5a52d20711a9bf0338813639755a3204da8f4): Don't treat `Prepended_Concatenation_Mark`s as zero width
- [5da00905](https://github.com/unicode-rs/unicode-width/commit/5da00905fc2db97eef2b601745a72ea7375df2d2): Give U+115F HANGUL CHOSEONG FILLER width 2
- [436b0db6](https://github.com/unicode-rs/unicode-width/commit/436b0db6c49e92657ab4b7f1098813fd08971303): Add more info to README
- [aae585fb](https://github.com/unicode-rs/unicode-width/commit/aae585fbd1a2bcaa621ea85b031bfbbe299007de): Mark interlinear annotation chars and Egyptian hieroglyph format controls as non-zero width
- [fda272b9](https://github.com/unicode-rs/unicode-width/commit/fda272b9bbc68c47466a62f751e66c4d2f0d096b): Merge pull request #34 from Jules-Bertholet/default-ignorable-code-point

  Fixes to characters considered zero-width
- [84a47fdc](https://github.com/unicode-rs/unicode-width/commit/84a47fdc1736c08ed3a3baa28d42ea72b12959a8): Doc: align no_std info with README

  unicode-width no longer depends on `std`. This has been updated in
  README.md in 96eaa4a8, but the introductory comments in `lib.rs` still
  show the old information.

  Align `lib.rs` with `README.md` and drop reference to the old `no_std`
  feature flag.
- [8803440e](https://github.com/unicode-rs/unicode-width/commit/8803440e61ac92ccce97c8c82bff748b0cf23fd2): Drop redundant use-declarations

  The package is always compiled as `no_std` and pulls in `std`  and
  `test` when compiling tests. Hence:

  - The prelude of `core` is available unconditionally, and it includes
    `core::options::Option` and its variants. No need to manually declare
    them.

  - The `test` crate is in-scope automatically, so no need to manually
    declare it.
- [c4c7ae57](https://github.com/unicode-rs/unicode-width/commit/c4c7ae57f4d11cf5aaa389d03d4877a75ffefbb8): Tests: drop redundant feature guards

  The `std` crate is unconditionally pulled in for test builds, hence we
  can rely on it. Drop the redundant guards that check against `no_std`.
- [3a57d02f](https://github.com/unicode-rs/unicode-width/commit/3a57d02fe9d630ddaa9c6eaa69770c31c3c637a3): Build: mark `no_std` as legacy feature flag

  The `no_std` feature flag has no effect, anymore (as already documented
  in `README.md`). Document this in `Cargo.toml`, but retain it for
  backwards compatibility.

  Note that for better cross-package composability, an `std` flag would
  likely be preferred in the future, over a `no_std` flag. The former
  allows multiple packages with different preferences on this feature to
  be combined in a single build, while the latter does not.

  Hence, suggest that `no_std` as feature flag is a legacy compatibility
  flag, and will not be used in the future.
- [7c489c3b](https://github.com/unicode-rs/unicode-width/commit/7c489c3b4ae1cadd52e8ee80aebeef447f7b969c): Merge pull request #36 from dvdhrm/pr/nostd

  nostd: remove left-overs from `no_std` feature flag
- [fdf5eb7a](https://github.com/unicode-rs/unicode-width/commit/fdf5eb7a4a10f8fd547661fd4f0b941bba0ce6f0): Ensure that canonically equivalent strings have the same width
- [9c4477c5](https://github.com/unicode-rs/unicode-width/commit/9c4477c52a113e6ceb51427bae03661622736441): Merge pull request #37 from Jules-Bertholet/canonical-equivalence

  Ensure that canonically equivalent strings have the same width
- [49ef0692](https://github.com/unicode-rs/unicode-width/commit/49ef06922e6b5353829017a896c6471dd9c4638b): Refactor tests

  - Convert into integration tests
  - Additional CI checks
- [00ee4b07](https://github.com/unicode-rs/unicode-width/commit/00ee4b070dbb0d2ce943ee1f1d4d7e562662a7fd): Merge pull request #38 from Jules-Bertholet/refactor-tests

  Refactor tests
- [8c0c8a10](https://github.com/unicode-rs/unicode-width/commit/8c0c8a10db7ff1eb8c0248f2bb1e6796474d3ae2): Document width rules
- [f7025021](https://github.com/unicode-rs/unicode-width/commit/f70250216033e8f797a4d133e797824b64fe5bc4): Cargo.toml: add categories
- [1e623c58](https://github.com/unicode-rs/unicode-width/commit/1e623c58b2dd7dbd8795bdf682ff1372fc0a8e44): Cargo.toml: make license SPDX-compliant
- [afd136af](https://github.com/unicode-rs/unicode-width/commit/afd136af9928d9134fa6d431159dea534dcdb19d): Merge pull request #40 from Jules-Bertholet/document-widths

  Document width rules
- [787fed3a](https://github.com/unicode-rs/unicode-width/commit/787fed3a29f008044337622ed2ed9b5f6c7d136b): `unicode.py`: Don't use `UnicodeData.txt` anymore
- [3885393a](https://github.com/unicode-rs/unicode-width/commit/3885393a0e5aca5252ac49548e9a4ae128f9bc30): Merge pull request #39 from Jules-Bertholet/no-more-unicodedata

  `unicode.py`: Don't use `UnicodeData.txt` anymore
- [6b503fa7](https://github.com/unicode-rs/unicode-width/commit/6b503fa75cb84b592633ffe22e65cd572744e430): Support emoji presentation sequences
- [73f816ed](https://github.com/unicode-rs/unicode-width/commit/73f816ed0d07d04c98a579906ffe23acef8783fd): Merge pull request #41 from Jules-Bertholet/emoji-presentation

  Support emoji presentation sequences
- [8092f84b](https://github.com/unicode-rs/unicode-width/commit/8092f84bddf454a763c711a86e534254e4dd8352): Bump to 0.1.12
## [0.1.11] - 2023-09-19

- [78cfe571](https://github.com/unicode-rs/unicode-width/commit/78cfe571e0e2848d218bffd26f80afee655c6209): Use Iterator::sum() instead of fold()

  This simplifies the code a tiny bit.
- [f5a9b4ef](https://github.com/unicode-rs/unicode-width/commit/f5a9b4efa5142d20d5f23070542e46e76f426a30): Merge pull request #32 from linkmauve/patch-1

  Use Iterator::sum() instead of fold()
- [8872449a](https://github.com/unicode-rs/unicode-width/commit/8872449ab3e5123b2f366141ec2089b6092048bb): Create rust.yml
- [a859939f](https://github.com/unicode-rs/unicode-width/commit/a859939f9179fd22f21f97d6c72df65e5c31168c): Update to Unicode 15.1
- [30e33bdb](https://github.com/unicode-rs/unicode-width/commit/30e33bdb2c9a576e1d0687347683cf34fc6245cf): Fixup script
- [24651a20](https://github.com/unicode-rs/unicode-width/commit/24651a204dcb6873f44b75f7ad645d1d2b0736bf): Merge pull request #33 from chrisduerr/unicode_15_1

  Update to Unicode 15.1
- [34fdd6b6](https://github.com/unicode-rs/unicode-width/commit/34fdd6b66e18ad385f341d0a51074b69d38a59d2): Add ignore file
- [89424871](https://github.com/unicode-rs/unicode-width/commit/89424871950f91033de79cd06318d8c06d94df84): Publish 0.1.11 (Unicode 15.1)
## [0.1.10] - 2022-09-13

- [ce07223d](https://github.com/unicode-rs/unicode-width/commit/ce07223db93f978be01d3cd5b46635956c9d59ea): Update to Unicode 14
- [b58e85b5](https://github.com/unicode-rs/unicode-width/commit/b58e85b5278309d7dd2ea514f7e6f9416b731b4d): Merge pull request #23 from chrisduerr/unicode_14

  Update to Unicode 14
- [0de9001b](https://github.com/unicode-rs/unicode-width/commit/0de9001b387f502109b1840f0c00fbcf57c262ba): Switch from binary search to lookup tables
- [088d7cb2](https://github.com/unicode-rs/unicode-width/commit/088d7cb2295a1bb4903a2c7202c11bc76dc53264): Add Wikipedia benchmarks & instructions
- [458f6ac3](https://github.com/unicode-rs/unicode-width/commit/458f6ac31408962b0065eafae72c6131174c99ca): Merge pull request #28 from mjguynn/multilevel-lut

  Improve performance, especially on non-ASCII codepoints
- [20050c68](https://github.com/unicode-rs/unicode-width/commit/20050c68ce42f08f801bd01e03e50ff2bf4ac4c9): Use static tables

  Using `static` instead of `const` reduces the size of output binaries, .rlib, and .rmeta.
- [c9f8c463](https://github.com/unicode-rs/unicode-width/commit/c9f8c463c224cbe27737d640dd4f43326810af70): Merge pull request #29 from mjguynn/master

  Change tables from const to static
- [3c67e3a2](https://github.com/unicode-rs/unicode-width/commit/3c67e3a287b4748ff4a165d50dcecbdd0382f08f): Update to Unicode 15
- [d6399a19](https://github.com/unicode-rs/unicode-width/commit/d6399a1917fb211666bc76c216f2bbc341a54660): Merge pull request #31 from chrisduerr/unicode_15

  Update to Unicode 15
- [f444a314](https://github.com/unicode-rs/unicode-width/commit/f444a314efeda9c130db6d6a96e925db0ea1ed13): Bump to 0.1.10
## [0.1.8] - 2020-06-29

- [c69f49cc](https://github.com/unicode-rs/unicode-width/commit/c69f49cc05eb0e238d5808fcc4d2d6936aa30a19): Update to Unicode 13

  Change version fields to u8 as the maximum value
  of each field is 255 according to specification.
- [a9fc0939](https://github.com/unicode-rs/unicode-width/commit/a9fc0939b14cdecbce2474966a511b5eb0ace2f9): Merge pull request #18 from pyfisch/unicode13

  Update to Unicode 13
- [d0aa5411](https://github.com/unicode-rs/unicode-width/commit/d0aa541144151c09268878dd781f06011498389a): Publish 0.1.8
## [0.1.17] - 2019-12-05

- [aa60704b](https://github.com/unicode-rs/unicode-width/commit/aa60704b9e5c25fd505f3e546ff160ee896667aa): Fix UAX11 links
- [ca01b2c1](https://github.com/unicode-rs/unicode-width/commit/ca01b2c1a56bda0235b944dbec6d8fe9ba8bbce6): Merge pull request #13 from vmedea/fixurl

  Fix UAX11 links in README.md
- [eec14d26](https://github.com/unicode-rs/unicode-width/commit/eec14d267230bc831f713d0af001c5cfbbefd284): Update to unicode 12.1

  This updates the tables.rs file to be compatible with the latest
  available unicode standard, allowing for new glyphs like the yawning
  face to be recognized properly.

  The unicode.py script has also been updated to python3, since that
  should be supported on more systems.
- [97b430a8](https://github.com/unicode-rs/unicode-width/commit/97b430a87eabdb501e56c4a798b888a4783d337d): Merge pull request #14 from chrisduerr/unicode-12

  Update to unicode 12.1
- [9e65826d](https://github.com/unicode-rs/unicode-width/commit/9e65826debf6c6e7e542ced5ff554b557fc3f432): Bump to 0.1.7
- [3033826f](https://github.com/unicode-rs/unicode-width/commit/3033826f8bf05e82724140a981d5941e48fce393): Merge pull request #16 from unicode-rs/bump

  Bump to 0.1.7
## [0.1.6] - 2019-08-19

- [ea86ecef](https://github.com/unicode-rs/unicode-width/commit/ea86ecef913fcc1cbeb2aaa4c0f189ce9213986d): Initial commit
- [c89e9d94](https://github.com/unicode-rs/unicode-width/commit/c89e9d94cc1ca219b1a339bcb761da6cf7912d4e): Add tablegen script and table
- [85050d8e](https://github.com/unicode-rs/unicode-width/commit/85050d8e415ce3c9ed4cfcc969a4c0427db35d40): First working version
- [3df193d1](https://github.com/unicode-rs/unicode-width/commit/3df193d1f49d9cda5dd8f3bcaee56f30bcb2bfe4): Add travis, README, update Cargo.toml
- [afabcb20](https://github.com/unicode-rs/unicode-width/commit/afabcb204b846a9b8bc862e2add56df76d23ae82): Move docs to the Traits
- [5a5364d8](https://github.com/unicode-rs/unicode-width/commit/5a5364d8e9ffdfdaef9c3ce8ba659e9ac249b853): Add COPYRIGHT and LICENSE files
- [efa04306](https://github.com/unicode-rs/unicode-width/commit/efa0430624acf7c821482d896ff45cd2a9658b57): Rename to unicode-width ; add tests from libcoretests for char.width()
- [2efee71c](https://github.com/unicode-rs/unicode-width/commit/2efee71cdcd50b19729cb6c82734348f51c4fe55): Tweak travis config to upload docs
- [6857232c](https://github.com/unicode-rs/unicode-width/commit/6857232c8278c7675820b7b7cdcbd6383071ec58): Fix some doc links
- [a9e64991](https://github.com/unicode-rs/unicode-width/commit/a9e6499160a54b52c39ee1c6a672d20205ced2f1): Fix Travis auto-refresh URL
- [4c51fb14](https://github.com/unicode-rs/unicode-width/commit/4c51fb14660f065f172956e7793bc69577b8ea39): Slight doc touch
- [ab525e66](https://github.com/unicode-rs/unicode-width/commit/ab525e6616548d74612c6a91684e367afdf5bd57): Make no_std a feature to allow builds in beta; bump version
- [1d27b56e](https://github.com/unicode-rs/unicode-width/commit/1d27b56ea3d92bef73d841a3c36530f0f5b891fb): Logo
- [cbad406d](https://github.com/unicode-rs/unicode-width/commit/cbad406d036f40c1f2d10eef65312a7be1d9dddb): Add benchmarks ; add explicit #[inline] annotations

  This diff adds benchmarks to get more info regarding Issue #1.

  It appears that the remaining difference between the "simple"
  case and the "cargo" case is the result of a difference in
  performance between using `match` and `if` for tight loops.

  I suspect it's because of the way that match arms get reordered:
  if I manually reorder the "if" statement, I can reproduce the
  match performance.

  Also added a couple #[inline] annotations in tables.rs, though
  the difference in performance in my measurements is negligible.

  Bumped version number to 0.1.1.
- [7f39d7c3](https://github.com/unicode-rs/unicode-width/commit/7f39d7c32c4f2004fde49e93410c3b895b770171): Add documentation link
- [b8c1ab9e](https://github.com/unicode-rs/unicode-width/commit/b8c1ab9e9007c05e316686ef88a5a5e5d771bba9): Merge pull request #3 from unicode-rs/doc-link

  Add documentation link
- [17466009](https://github.com/unicode-rs/unicode-width/commit/174660098c2a1a8fd7b37b7f8a5b32c18743f27c): Update to Unicode 8.0.0; update Cargo.toml
- [4d298126](https://github.com/unicode-rs/unicode-width/commit/4d2981261cd9f82d3d02231a9a41801afe6bd28e): Update .travis.yml for explicit nightly rust
- [cff7514a](https://github.com/unicode-rs/unicode-width/commit/cff7514a3fe3c1458078435fc3406a8572b48110): Add necessary feature gates for no_std feature
- [1c424cfd](https://github.com/unicode-rs/unicode-width/commit/1c424cfd93c2002d4979a043138afda89881e696): Update no_std feature to compile with nightly.
- [c511c16c](https://github.com/unicode-rs/unicode-width/commit/c511c16c5084728a0d64db61c3728152916b9f73): Update Cargo.toml example to correct version number
- [96eaa4a8](https://github.com/unicode-rs/unicode-width/commit/96eaa4a88186fdbc78adfe8d4eba9299b09528e5): Update to Unicode 9 ; clean up no_std and related
- [4940d3fe](https://github.com/unicode-rs/unicode-width/commit/4940d3fe277a6de0ae640411d7d16ec1c76dbbc5): Update tables.rs to Unicode 10
- [26d1bfbb](https://github.com/unicode-rs/unicode-width/commit/26d1bfbb774b0eadb3f81f3c790f9ec3eaef8600): Update version to 0.1.5
- [62956a98](https://github.com/unicode-rs/unicode-width/commit/62956a983e4f2f82f5f38a8745f820d40fc89ac5): Test ugly travis workaround
- [9bc53750](https://github.com/unicode-rs/unicode-width/commit/9bc537504a26e777c3622835bb546acb4008451d): Update access token for gh_pages updates
- [6602390a](https://github.com/unicode-rs/unicode-width/commit/6602390a06e0ef7d57798843fb4adee02a9a834c): Add a possible issue to the README.
- [88bc3eaf](https://github.com/unicode-rs/unicode-width/commit/88bc3eafd69c93473a68b0e9ed9d03359ad4e5ff): Add comment in README linking to UAX #11 for further details.
- [ff4923ab](https://github.com/unicode-rs/unicode-width/commit/ff4923ab40d1bc13922b049a0ad04c61b447345d): Merge pull request #7 from theindigamer/add-emoji-example

  Add a possible issue to the README.
- [a32845f0](https://github.com/unicode-rs/unicode-width/commit/a32845f0215391f118a674868a65d78adbd982d0): Update support for being built in rust-lang/rust

  This commit updates the support necessary for this crate to be built as
  a dependency of `getopts` which is a dependency of the `test` crate
  which is built in rust-lang/rust. This change will be required to land
  rust-lang/rust#63637 where Rust's build system is being tweaked
  slightly. This is intended to not have any impact on other users of this
  crate and the `Cargo.toml` features here should only be used by Rust's
  own build system.
- [276a930f](https://github.com/unicode-rs/unicode-width/commit/276a930f267dc51f78a74fff1bcc28e338935613): Merge pull request #12 from alexcrichton/update-support

  Update support for being built in rust-lang/rust
- [3a222d1d](https://github.com/unicode-rs/unicode-width/commit/3a222d1dd192a4cb7877ba88f8bb317d8628bc1b): Bump to 0.1.6
<!-- generated by git-cliff -->
