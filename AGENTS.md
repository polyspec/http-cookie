# AGENTS.md

Rules for every change of http-cookies. The rules of the library are in [docs/spec/cookies.md](docs/spec/cookies.md).

## Layout

- One Cargo workspace at the root (`Cargo.toml`, `Cargo.lock`) with the core and the three binding crates; the lints
  are those of the root `Cargo.toml` and `clippy.toml`.
- `packages/rust/`: the crate `polyspec-http-cookies`, which holds every rule (CK-1.4). It builds and tests without the
  generator. `packages/rust/interface.json`: the interface contract of the bindings (CK-1.5). The crate package holds
  it (`include` of `packages/rust/Cargo.toml`), so that a repository that depends on the crate reads it through Cargo.
- `packages/python/`, `packages/ts/`, `packages/php-ext/`: the bindings for Python, Node and PHP. Each is a thin package
  that `bindgen scaffold` writes and that is tracked: `Cargo.toml`, `src/lib.rs`, which includes `generated/lib.rs`,
  and the manifest of the language (`pyproject.toml`, `package.json`, `composer.json`). They are not edited by hand.
  `generated/` of each is written by bindgen and is not tracked.
- `contracts/cookies.json`: the cases of the rules (CK-14).
- `docs/spec/`: the specification. Root: `AGENTS.md`, `README.md`, `CHANGELOG.md` and their Korean translations
  (`*.ko.md`), `Makefile`, `rust-toolchain.toml`, `clippy.toml`, `rustfmt.toml`.
- There is no implementation in another language to remove or to keep beside the bindings.

## Bindings

- `polyspec-bindgen-core` is a Git dependency of the core and of the binding crates (branch `v1/rust-core` of the
  bindgen repository) until bindgen is released; `Cargo.lock` pins its commit. The core implements
  `bindgen_core::Kinds` for its error (CK-13.2, CK-13.3).

## Changes

- A change of a rule changes the specification first, then the cases of `contracts/cookies.json`, then the code. A
  test names the rule statement that it checks (`CK-n.m`).
- A rule states what it requires and its basis: a rule id of this library or a section of a standard (RFC 6265, RFC
  9110, RFC 9113, draft-ietf-httpbis-rfc6265bis) or the behavior of a peer, such as a browser. A tracked file does not
  record where a rule was taken from or who asked for it.
- A part of an input that gives nothing is a row of the table "Inputs that give no cookie" of the specification, with
  its rule, its basis and a case; the code names the rule where it leaves the bytes. An ignored input without a row is a
  defect.
- An invalid argument fails with a kind; the library never turns it into a plausible value.
- The specification, the cases, the README, this file and the change log (English and Korean) change in the same
  change as the code.

## Rust

- The toolchain is the release that `rust-toolchain.toml` pins. `make check` runs clippy with `-D warnings`, rustfmt
  and the tests.
- The lints of the workspace (`Cargo.toml` at the root) deny every warning, unsafe code, `unwrap`, `expect`, `as` conversions,
  `let _` of a value that must be used or of an untyped value, and an allow attribute without a reason.
  `clippy.toml` disallows `Option::{unwrap_or, unwrap_or_default, unwrap_or_else, map_or, map_or_else}` and
  `Result::{ok, map_or, map_or_else}`. A use that a rule needs carries `#[allow(..., reason = "<rule id>")]`.
- The crate does no I/O and reads no clock, environment or random source (CK-1.2).

## Platforms

Linux x64 and arm64 (glibc) and macOS arm64. Windows is not supported until the Rust cores and the bindings of polyspec
are stable and ext-php-rs builds on Windows with a pinned stable toolchain.

## Commits

- A commit message is `<type>(<scope>): <Subject>`, factual and concise, with a body that states what changed and why.
  Unrelated changes are separate commits.
- No tracked file names a Git commit.
