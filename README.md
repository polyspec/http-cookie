# http-cookies

[한국어](README.ko.md)

Reads the cookies of a request from its `Cookie` field lines and writes the field value of one `Set-Cookie` field
line, by one Rust implementation. Both are pure functions. The rules are in [docs/spec/cookies.md](docs/spec/cookies.md);
the cases that every implementation passes are in [contracts/cookies.json](contracts/cookies.json).

- Reading (CK-2 to CK-5): the lines split at `;` into pairs, spaces and tabs at both ends of a pair are removed, a pair
  splits at its first `=`, the value is percent-decoded, and the first cookie of a name counts. A pair that cannot be a
  cookie (no `=`, an empty name, a name or value that is not UTF-8) gives none, and reading never fails.
- Writing (CK-6 to CK-12): `<name>=<value>; Max-Age=<n>; Path=<path>; Secure; HttpOnly; SameSite=<value>`, in this
  order, with the attributes that the caller states. The name must be a token, the value is percent-encoded so that
  reading gives it back, and a cookie that a browser would drop (a bad path, more than 4096 bytes, a `__Secure-` or
  `__Host-` name without its attributes, `SameSite=None` without `Secure`) fails with a kind.

## Rust

```rust
use polyspec_http_cookies::{parse_cookie, set_cookie, Attributes, SameSite};

let cookies = parse_cookie(&[b"a=1; b=%C3%A9".to_vec()]);
assert_eq!(cookies, vec![("a".into(), "1".into()), ("b".into(), "é".into())]);

let attributes = Attributes { path: "/".into(), max_age: Some(60), secure: true, http_only: true, same_site: Some(SameSite::Lax) };
assert_eq!(set_cookie("__Host-id", "v", &attributes)?, b"__Host-id=v; Max-Age=60; Path=/; Secure; HttpOnly; SameSite=Lax");
```

PHP, Python and Node bindings will be generated from [contracts/interface.json](contracts/interface.json), which is a
draft; there is no binding yet.

## Checks

```
make check   # clippy -D warnings, rustfmt --check and the tests
```

Requirements: the Rust release of `rust-toolchain.toml`. Windows is not supported yet.
