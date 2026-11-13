<!-- doc-id: agents -->
# AGENTS.md

Rules for every change of http-cookie. The rules of the library are in [docs/spec/cookies.md](docs/spec/cookies.md).

## Layout

- One Cargo workspace at the root (`Cargo.toml`, `Cargo.lock`) with the core and the binding crates of the five
  targets; the lints are those of the root `Cargo.toml` and `clippy.toml`.
- `packages/rust/`: the crate `polyspec-http-cookie`, which holds every rule (CK-1.4). It builds and tests without the
  generator. `packages/rust/interface.json`: the interface contract of the bindings (CK-1.5). The crate package holds
  it (`include` of `packages/rust/Cargo.toml`), so that a repository that depends on the crate reads it through Cargo.
- `packages/python/`, `packages/ts/`, `packages/ts/wasm/`, `packages/php-ext/`, `packages/go/`: the bindings for Python,
  Node, the browser (WASM), PHP and Go, the five targets that every contract names (binding BD-27); the C ABI of Go is
  the crate `packages/go/internal/ffi`. Each is a thin package
  that `binding scaffold` writes and that is tracked: `Cargo.toml`, `src/lib.rs`, which includes `generated/lib.rs`,
  the `build.rs` of the Node addon, and the manifest of the language (`pyproject.toml`, `package.json`,
  `composer.json`, `go.mod`). They are not edited by hand; `make bindings-check` fails when they differ from what binding
  writes. `generated/` of each (`packages/go/internal/ffi/generated` and `packages/ts/wasm/generated` included) is
  written by `make bindings` and is not tracked. In the Go module binding owns the files `zz_generated_*.go` instead of
  a directory, and they are tracked (binding GN-3), because `go get` takes a module from its tracked files; `make
  bindings` writes them again. `tests/` of each holds the runner of the contract cases in that language (CK-14.5).
- `contracts/cookies.json`: the cases of the rules (CK-14).
- `scripts/binding.mts`: runs binding (see "Bindings").
- `docs/spec/`: the specification. Root: `AGENTS.md`, `README.md`, `CHANGELOG.md` and their Korean translations
  (`*.ko.md`), `Makefile`, `rust-toolchain.toml`, `clippy.toml`, `rustfmt.toml`, `.cargo/config.toml`, and
  `package.json`, `package-lock.json`, `composer.json` and `composer.lock` of the development tools, and `mise.toml`,
  `mise.lock`, `config/toolchain.json`, `.php-version`, `.npmrc` and `.mise/locks/` of the tools of the checkout (see
  "Bindings").
- There is no implementation in another language to remove or to keep beside the bindings.

## Bindings

- The repository is one Cargo workspace: the core `packages/rust` and the binding crates of the five targets
  (binding BD-27) `packages/python`, `packages/ts`, `packages/ts/wasm`, `packages/php-ext` and
  `packages/go/internal/ffi` (with `packages/go/go.mod`), which `binding scaffold` writes and `make bindings-check`
  compares; their `generated/` is not tracked. `Cargo.lock` at the root pins every dependency.
- `polyspec-binding-core` is a Git dependency of the core and of the binding crates (branch `v1/rust-core` of the
  binding repository) until binding is released; `Cargo.lock` pins its commit. The core implements
  `binding_core::Kinds` for its error (CK-13.2, CK-13.3).
- binding itself is the TypeScript tool of the same repository, which no registry holds yet. `scripts/binding.mts` takes
  it from the checkout that Cargo made of the commit that `Cargo.lock` pins, clones it into `var/binding/<commit>` and
  installs its dependencies there by its `package-lock.json`, so that the generator and `binding_core` are one version.
  The clone is a Git checkout of binding at that commit: binding check reads its own commit from it and compares it with
  the commit of `Cargo.lock` (CK-20), which a copy of the files alone fails. This holds only until the first release of
  binding.
- A case of `contracts/cookies.json` follows the case convention of binding (G-5): its keys are the parameter names
  in snake_case (`lines`, `name`, `value`, `attributes`), and a record is the object of its fields (the attributes are
  `{"path", "max_age", "secure", "http_only", "same_site"}`).
- `make bindings` generates; every make target that builds or tests a binding runs it first. `make test-bindings` runs
  clippy on the whole workspace, `make check-declarations`, then `make test-python`, `make test-node`, `make
  test-wasm`, `make test-php` and `make test-go`, one for each of the five targets of the contract, each of which runs
  the tests that binding generates and the contract cases of the binding on the standard runner of its language:
  unittest, node --test (Node and WASM, the WASM binding through the subpath `./wasm` of the same installed package),
  PHPUnit and go test. The contract cases are `packages/python/tests`, `packages/ts/tests/cases.test.mjs`,
  `packages/ts/tests/cases.wasm.test.mjs`, `packages/php-ext/tests` and `packages/go/tests`. A target that the
  generator cannot take stays a failing target with its cause; no target is left out or skipped (binding BD-27).
- In the Go module binding owns the files `zz_generated_*.go` (binding GN-3); they are tracked, because a Go module is
  fetched as source, and `make bindings` writes them again. `packages/ts/wasm/generated` and
  `packages/go/internal/ffi/generated` are not tracked; wasm-bindgen writes the module and its glue into the first.
  The variables `PYTHON` (3.14 or later), `PHP`, `PHP_CONFIG`, `PHPUNIT`, `GO`, `GOFMT` and `WASM_BINDGEN` name the
  runtimes. `make test-go` fails a Go of another release than the go directive of `packages/go/go.mod` and runs it with
  `GOTOOLCHAIN=local`, so that it fetches no toolchain, and `make test-wasm` fails a wasm-bindgen CLI of another release
  than the `wasm-bindgen` crate of `packages/ts/wasm/Cargo.toml`, because the CLI reads only a module of its own
  release (binding GN-18). `CARGOFLAGS` adds
  arguments to every cargo build. On macOS a PHP extension and a Node addon take the symbols of their runtime when it
  loads them, so `.cargo/config.toml` tells the linker to leave them undefined, and the recipes name the library that
  Cargo builds by the extension of the platform (`dylib` on macOS, `so` on Linux).
- The tools are pinned in tracked files and installed into `var/tools` of the checkout, as in binding: `mise.toml`
  names Node.js, npm, Go, Python, Composer and the wasm-bindgen CLI at the releases of binding and `mise.lock` their
  digests, `config/toolchain.json` the release of mise and its digests, `rust-toolchain.toml` the Rust toolchain with
  the target `wasm32-unknown-unknown` and `.php-version` the PHP of the machine, which the wrappers of `var/tools/bin`
  run. A fresh checkout runs `make bootstrap`, which installs `package-lock.json` with the npm of `devEngines` of
  `package.json`, then `make install`, which runs the installers of `@polyspec/kit` (`make install-tools`). Every recipe
  runs with `var/tools/bin` and the shims of mise first on `PATH`; `GO`, `GOFMT` and `WASM_BINDGEN` name the shims of
  mise and `PHP` and `PHP_CONFIG` the wrappers of `var/tools/bin`.
- The development tools are dependencies of the repository: TypeScript and `@polyspec/kit` in `package.json`, pinned by
  `package-lock.json`, and PHPUnit in `require-dev` of `composer.json`, installed into `vendor/` by `composer install`
  and pinned by `composer.lock`.

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
- Every tracked document is an English file `X.md` and its Korean translation `X.ko.md`, which say the same sentence
  by sentence, so that a reader of either language reads the same rules. Both start with the same line
  `<!-- doc-id: <name> -->`, and the second line of the Korean file is `<!-- source-sha256: <hash> -->` with the sha256
  of the English file (`shasum -a 256`). A change of one file changes the other and writes the hash again. No make
  target checks the pairs.

## Rust

- The toolchain is the release that `rust-toolchain.toml` pins. `make check` runs clippy with `-D warnings` and the
  tests of the core, rustfmt on the whole workspace (which needs no generated file), and `binding check` (the
  contract and the tracked binding packages).
- The lints of the workspace (`Cargo.toml` at the root) deny every warning, unsafe code, `unwrap`, `expect`, `as`
  conversions, `let _` of a value that must be used or of an untyped value, and an allow attribute without a reason.
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
