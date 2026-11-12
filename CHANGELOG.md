# Changelog

[한국어](CHANGELOG.ko.md)

## Unreleased

- Added the specification `docs/spec/cookies.md` (CK-1 to CK-14), the cases `contracts/cookies.json` and the Rust crate
  `polyspec-http-cookies` with `parse_cookie` (the `Cookie` field lines of a request read into ordered name and value
  pairs) and `set_cookie` (one `Set-Cookie` form, with the error kinds `invalid_name`, `invalid_cookie_path`,
  `too_large`, `prefix_not_met` and `same_site_none_not_secure`).
- Added the draft interface contract `contracts/interface.json`. No binding is generated yet.
- Windows is not supported.
