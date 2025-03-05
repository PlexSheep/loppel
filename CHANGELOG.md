# Changelog

## [Unreleased]

## [0.1.1](https://github.com/PlexSheep/loppel/compare/v0.1.0...v0.1.1)

### ⛰️ Features

- Remove original with -d - ([1f2ae1b](https://github.com/PlexSheep/loppel/commit/1f2ae1bd1ea5739ee37a97b67127eca6f04840ae))

### 📚 Documentation

- Update readme and Cargo.toml (keywords and categories) - ([0cabaf0](https://github.com/PlexSheep/loppel/commit/0cabaf0b588d207ab7c31030c7e60325477863c5))

### ⚙️ Miscellaneous Tasks

- Fix typo in readme - ([cef26fd](https://github.com/PlexSheep/loppel/commit/cef26fd1cf03610f7abe379095890139b534edb2))
- Allow publish - ([55f5019](https://github.com/PlexSheep/loppel/commit/55f50196cc39e4054089fc16a87a75602d246ddf))


## [0.1.0]

### ⛰️ Features

- Remove original with -d - ([1f2ae1b](https://github.com/PlexSheep/loppel/commit/1f2ae1bd1ea5739ee37a97b67127eca6f04840ae))
- Add --output for restore - ([0424815](https://github.com/PlexSheep/loppel/commit/0424815cdb05161793875de2f56dfeaea3e75d83))
- Restore if file has bak in name - ([ec91cdf](https://github.com/PlexSheep/loppel/commit/ec91cdfd2e8969e92d261497dbf8571542e71ad9))
- Restore bak.d dirs - ([d43cc85](https://github.com/PlexSheep/loppel/commit/d43cc853e3659658b7b5fabac2f7e83705790210))
- Restore simple files - ([cfbcf70](https://github.com/PlexSheep/loppel/commit/cfbcf70e5e4e60a3ddda4520120aa13c833a3cf5))
- Implement a helper function to be able to read archives - ([fb29cb5](https://github.com/PlexSheep/loppel/commit/fb29cb50cd39918adc22498df53c122ae597d8fb))
- File compressions with tar - ([bedffe3](https://github.com/PlexSheep/loppel/commit/bedffe342fb5641f336df8e7cf16aa3c6e26603f))
- Basic functionality, cli default command did not work in some cases - ([160b487](https://github.com/PlexSheep/loppel/commit/160b4879412772980ae915d7c2bebf2f4aa24372))
- Cli interface - ([174015b](https://github.com/PlexSheep/loppel/commit/174015b2a2071ab23c7931c81d61700f73067506))

### 🐛 Bug Fixes

- Always restore to current directory - ([73e428a](https://github.com/PlexSheep/loppel/commit/73e428a5a04600e457915ef52a700bfbf172b1e8))
- Unpack archive into a dir with the fitting name (suffix removed) - ([8e591b4](https://github.com/PlexSheep/loppel/commit/8e591b4d98444beb000882fd4ef6d2ef149b3378))
- Restore if just given *bak - ([822f354](https://github.com/PlexSheep/loppel/commit/822f354a043755acd8fb14274f2aee4026d77559))
- Typo in Cargo.toml - ([1df4ffa](https://github.com/PlexSheep/loppel/commit/1df4ffae48712b4c8b14ff3c3fdf43643b832815))
- Restore did not check if path actually exists - ([7c1e875](https://github.com/PlexSheep/loppel/commit/7c1e8759c725c3c81715abde7c176d5921008056))
- Recursive remove not only for dirs - ([bf74382](https://github.com/PlexSheep/loppel/commit/bf743829775714fa20c2069352f9ecf0a8811797))
- Confirm had bad formatting and bad reading - ([338fdb6](https://github.com/PlexSheep/loppel/commit/338fdb66e2855b2a4b008de10506ee102a0c3a0d))
- Zstd encoder did not finish before - ([bc86028](https://github.com/PlexSheep/loppel/commit/bc8602890bf82d46292badef1288d0cf9661fc3b))
- Cli default workaround had another problem - ([f63c976](https://github.com/PlexSheep/loppel/commit/f63c9762d171279994010126380bda77cd0d811d))
- Adding extension just overwrote the extension - ([1144f1a](https://github.com/PlexSheep/loppel/commit/1144f1a4fb6d93912d06bfc03e7f045ce78286fa))

### 🚜 Refactor

- Many small things - ([12f7c1d](https://github.com/PlexSheep/loppel/commit/12f7c1dfd822adbd902fbea1a7853b9ab702224b))
- Get rid of invisible command alias - ([10d256b](https://github.com/PlexSheep/loppel/commit/10d256b554c2b5c8b32420290e162992f029bd2a))
- Test_make_archive - ([aa5d66d](https://github.com/PlexSheep/loppel/commit/aa5d66d79055604d6988e7340ccbbfcf39ef2a72))

### 📚 Documentation

- Update readme and Cargo.toml (keywords and categories) - ([0cabaf0](https://github.com/PlexSheep/loppel/commit/0cabaf0b588d207ab7c31030c7e60325477863c5))
- Typo in readme - ([aedea2f](https://github.com/PlexSheep/loppel/commit/aedea2f9f79f14076e70985f796631b257f575d4))

### 🧪 Testing

- Bak.d testing - ([9202a3c](https://github.com/PlexSheep/loppel/commit/9202a3c7f622418ecfdf7300d02b5aabdac7c491))
- Refactor tests - ([f730d91](https://github.com/PlexSheep/loppel/commit/f730d911ceb26557ee841e1abad7e593ed9623a9))
- Fix race condition in tests - ([9ab884a](https://github.com/PlexSheep/loppel/commit/9ab884ae7d064c3e0bf271d238cf68c6d581237f))
- Add tests for simple backups - ([6778349](https://github.com/PlexSheep/loppel/commit/6778349460d1176ed53f37bd896fa5cf153528df))
- Find out how to properly use tar-rs - ([20973f5](https://github.com/PlexSheep/loppel/commit/20973f50ebb811c2e3dbf4c889d152711e5c114a))
- Test the dependency - ([6ee7e0a](https://github.com/PlexSheep/loppel/commit/6ee7e0ad988c382d8107b36f207cb177c4bee11d))

### ⚙️ Miscellaneous Tasks

- Allow publish - ([55f5019](https://github.com/PlexSheep/loppel/commit/55f50196cc39e4054089fc16a87a75602d246ddf))
- Ci now runs less often - ([8ed1c55](https://github.com/PlexSheep/loppel/commit/8ed1c5508f762033e23b5cae5f1dd658705332d5))
- Disable audit.yaml because it doesnt work apperently - ([23dc776](https://github.com/PlexSheep/loppel/commit/23dc7763c0c7f6558c7aac5d8317f9ca9167b741))
- Update lockfile - ([873854e](https://github.com/PlexSheep/loppel/commit/873854eeb23193fa7dcf8e3b6c909f85a5783ba7))
- Automatic Rust CI changes - ([8af6346](https://github.com/PlexSheep/loppel/commit/8af6346841da8c4e8c4b637511e2fbf6e1966ca2))
- Rename to loppel - ([ad41f96](https://github.com/PlexSheep/loppel/commit/ad41f965046f4475bb254d1abfa572ed36676730))
- Specify that github actions commits the cargo ci stuff - ([bf2671e](https://github.com/PlexSheep/loppel/commit/bf2671e90b3757866b9e3d6264845b40b8852468))
- Super basic readme, naming, license - ([bebc6f1](https://github.com/PlexSheep/loppel/commit/bebc6f1ea9a61353cb6813e69e7c8d4f607497c7))
- Allow cargo ci to write - ([a7c31f5](https://github.com/PlexSheep/loppel/commit/a7c31f5a0109867d3702c708bcd8a5883bd441cc))
- Fix cargo ci - ([ff293a1](https://github.com/PlexSheep/loppel/commit/ff293a16732c300a7673e350564786f231e6d4a4))

