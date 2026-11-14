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
  a directory (binding GN-3); `make bindings` writes them and they are not tracked either (see "Bindings"). `tests/` of each holds the runner of the contract cases in that language (CK-14.5).
- `contracts/cookies.json`: the cases of the rules (CK-14).
- `tests/scripts/`: the tests of the tracked files, which `make
  test-scripts` runs.
- `docs/spec/`: the specification. `docs/plans/rust-core/`: the plan and its checklist, which `make docs-check` reads
  with `config/documents.json` and `config/checklist.json` (see "Changes"). Root: `AGENTS.md`, `README.md`, `CHANGELOG.md` and their Korean translations
  (`*.ko.md`), `Makefile`, `rust-toolchain.toml`, `clippy.toml`, `rustfmt.toml`, `.github/` (see "CI and release"),
  `config/release.json`, and `package.json`, `package-lock.json`, `composer.json` and `composer.lock` of the development
  tools, and `mise.toml`, `mise.lock`, `config/toolchain.json`, `.php-version`, `.npmrc` and `.mise/locks/` of the tools
  of the checkout (see "Bindings").
- There is no implementation in another language to remove or to keep beside the bindings.

## Bindings

- The repository is one Cargo workspace: the core `packages/rust` and the binding crates of the five targets
  (binding BD-27) `packages/python`, `packages/ts`, `packages/ts/wasm`, `packages/php-ext` and
  `packages/go/internal/ffi` (with `packages/go/go.mod`), which `binding scaffold` writes and `make bindings-check`
  compares; their `generated/` is not tracked. `Cargo.lock` at the root pins every dependency.
- `polyspec-binding-core` is a Git dependency of the core and of the binding crates on the tag `v0.0.2` of the binding
  repository, its release; `Cargo.lock` pins the commit of that tag. The core implements `binding_core::Kinds` for its
  error (CK-13.2, CK-13.3).
- binding itself is the npm package `@polyspec/binding` of the same release, which `package.json` names by the URL of
  its release asset and `package-lock.json` pins by its integrity, so that the generator and `binding_core` are one
  version (G-6); binding check takes the tag of `Cargo.lock` as its own (CK-20). The recipes run
  `node_modules/@polyspec/binding/dist/cli.js` with the node of `var/tools/bin`. The bootstrap and `make
  install-packages` install it, and `make install-binding` fetches the crates of `Cargo.lock` (`cargo fetch
  --locked`), which binding reads with `cargo metadata --locked --offline`; `make bindings-check` and `make bindings`
  download nothing and fail while they are missing, because a check that downloads depends on the network.
  `tests/scripts/binding.test.mts` requires the tag, the asset, its integrity and these recipes.
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
- In the Go module binding owns the files `zz_generated_*.go` (binding GN-3), which `make bindings` writes. They are
  not tracked, and neither is a static library of `packages/go/internal/ffi/lib/<platform>`: a release adds both in the
  commit of its Go tag, because Go fetches a module as the source of a tag (binding GN-15g, CK-19), and a tracked copy
  would differ from what the pinned binding writes after every change of the pin. `packages/ts/wasm/generated` and
  `packages/go/internal/ffi/generated` are not tracked; wasm-bindgen writes the module and its glue into the first.
  The variables `PYTHON` (3.14 or later), `PHP`, `PHP_CONFIG`, `PHPUNIT`, `GO`, `GOFMT` and `WASM_BINDGEN` name the
  runtimes. `make test-go` fails a Go of another release than the go directive of `packages/go/go.mod` and runs it with
  `GOTOOLCHAIN=local`, so that it fetches no toolchain, and `make test-wasm` fails a wasm-bindgen CLI of another release
  than the `wasm-bindgen` crate of `packages/ts/wasm/Cargo.toml`, because the CLI reads only a module of its own
  release (binding GN-18). `CARGOFLAGS` adds
  arguments to every cargo build. On macOS a PHP extension and a Node addon take the symbols of their runtime when it
  loads them, so the linker must leave them undefined: the `build.rs` that binding scaffold writes for each of the two
  crates says so for its library alone (binding GN-3d), and no tracked file gives a link setting in `RUSTFLAGS` or
  `.cargo/config.toml`. The recipes name the library that Cargo builds by the extension of the platform (`dylib` on
  macOS, `so` on Linux).
- The tools are pinned in tracked files and installed into `var/tools` of the checkout, as in binding: `mise.toml`
  names Node.js, npm, Go, Python, Composer, the wasm-bindgen CLI and maturin at the releases of binding and `mise.lock` their
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
  of the English file. A change of one file changes the other, and `make documents-stamp PATHS=<twin>` writes the hash
  again after the translation is reviewed; no marker is written by hand.
- `make docs-check` reads each document that `config/documents.json` selects, with its links, anchors and fences and
  the `doc-id` and `source-sha256` of each Korean twin, and the checklist `docs/plans/rust-core/checklist.md`, whose
  rows `config/checklist.json` declares. The plan of the repository is `docs/plans/rust-core/plan.md`; a task is a row
  of that checklist, marked `[o]` only after its verification command passed on the committed tree. Run it before a
  commit.

## Rust

- The toolchain is the release that `rust-toolchain.toml` pins. `make check` runs clippy with `-D warnings` and the
  tests of the core, rustfmt on the whole workspace (which needs no generated file), `binding check` (the
  contract and the tracked binding packages), `make docs-check` and `make test-scripts`.
- The lints of the workspace (`Cargo.toml` at the root) deny every warning, unsafe code, `unwrap`, `expect`, `as`
  conversions, `let _` of a value that must be used or of an untyped value, and an allow attribute without a reason.
  `clippy.toml` disallows `Option::{unwrap_or, unwrap_or_default, unwrap_or_else, map_or, map_or_else}` and
  `Result::{ok, map_or, map_or_else}`. A use that a rule needs carries `#[allow(..., reason = "<rule id>")]`.
- The crate does no I/O and reads no clock, environment or random source (CK-1.2).

## CI and release

- `.github/workflows/ci.yml` runs on the push of `main` and on a manual run, one job per matrix entry: `core` runs
  `lint-rust`, `fmt-check`, `test`, `bindings-check`, `docs-check`, `test-scripts` and `release-coverage`, and `bindings` runs the targets of `make
  test-bindings`. Each job runs them with `make ci-targets`, which runs every target to its end and writes the report
  `var/ci/<name>/`, and the last job `ci-passed` passes only when every other job passed. The jobs run on
  `ubuntu-26.04-arm`, pin every action by its commit, set no `timeout-minutes` and run every command of a step as one
  make target, so that the settings of the Makefile apply to it. `.github/actions/bootstrap` is the bootstrap of every
  job, and `make install-php-tools` installs PHPUnit of `composer.lock` into `vendor/` with the Composer of `mise.toml`.
- `config/release.json` classifies every package file, which `make release-coverage` checks. A tag `vX.Y.Z` releases the
  core crate `packages/rust` as a Git tag, which a Cargo dependency names with `tag = "vX.Y.Z"`, and it is the version
  of the binding manifests, which binding scaffold writes from the version of the core. The release also tags the Go module
  `packages/go` with `packages/go/vX.Y.Z` at the release commit (`goModules` of `config/release.json`), because Go
  takes a module below the root of a repository by the tag of that form, and a repository whose Go binding names a
  record of this one requires that module at that version (binding GN-15g). A tag releases no archive yet: a binding
  is built per platform (an abi3 wheel, a PHP extension, an npm package with its platform packages, a Go static
  library), and the release utility of kit 0.0.109 packs only npm packages, Composer source archives and pure Python
  wheels.
- A release is the commit of a successful push or manual run of `ci.yml` on `main`. The version-bump commit
  `chore(release): Release X.Y.Z` sets the version of the core (`version` of `packages/rust/Cargo.toml`), writes the
  binding manifests again with binding scaffold (`node node_modules/@polyspec/binding/dist/cli.js scaffold
  --contract packages/rust/interface.json`), updates the versions of the local packages in `Cargo.lock` and renames `## Unreleased`
  of both changelogs to `## X.Y.Z`, with a new empty `## Unreleased` above it. When the CI run of that push succeeds,
  `.github/workflows/release.yml` starts by itself, and `gh workflow run release.yml -f run=<id>` starts it again: `make
  release-tag` creates and pushes the annotated tag `vX.Y.Z` at that commit, and `make release-publish` and `make
  release-undraft` create the GitHub Release. No tag is created by hand.
- Open before the first release, which is not complete without them: `origin` has no `main`, so the release pushes
  `v1/rust-core` as `main` and makes `main` the default branch; and the packages of the bindings (the abi3 wheel, the
  PHP extension, the npm package with its platform packages) are rows of `config/release.json` only when kit 0.0.111
  and a release of binding exist, because kit 0.0.109 refuses their kinds.

## Platforms

Linux x64 and arm64 (glibc) and macOS arm64. Windows is not supported until the Rust cores and the bindings of polyspec
are stable and ext-php-rs builds on Windows with a pinned stable toolchain.

## Commits

- A commit message is `<type>(<scope>): <Subject>`, factual and concise, with a body that states what changed and why.
  Unrelated changes are separate commits.
- No tracked file names a Git commit.
