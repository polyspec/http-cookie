<!-- doc-id: changelog -->

- The Go files that binding generates are tracked and the other generated directories of Go and WASM are ignored:
  `packages/go/zz_generated_*.go`, `packages/go/errors/` and `packages/go/internal/binding/` hold the files
  `zz_generated_*.go` that `make bindings` writes, which binding owns in the Go module and which are tracked (binding
  GN-3), so that `go get` takes the module from its tracked files. `.gitignore` names
  `packages/go/internal/ffi/generated/` and `packages/ts/wasm/generated/`. Before, `make bindings` left these files and
  directories untracked and not ignored. The end-to-end check of binding (`make e2e` of binding at the merged commit
  of `BD-27`) runs the five targets of the contract on their runners and passes each: PHP on PHPUnit, Python on
  unittest, Node and WASM on `node --test`, Go on `go test`, with no skipped test.

- The contract names the five targets of binding (BD-27): `targets` of `packages/rust/interface.json` is php, python,
  node, go and wasm, `packages.go` names the Go module `github.com/polyspec/http-cookie/packages/go`, the entry `.` of
  the npm package takes the conditions `{node: napi, browser: wasm}`, each error class names its WASM class, the Node
  one, and `errors.fixed` names `TypeError` for WASM as for Node. Before, the contract named php, python and node, so
  no Go or WASM binding was generated. The tracked binding packages gain what binding scaffold writes for the two
  targets (`packages/go/go.mod`, `packages/go/internal/ffi`, `packages/ts/wasm`, the browser exports of
  `packages/ts/package.json`), and both crates are members of the workspace. The contract check has no finding. No make
  target builds or tests the Go and WASM bindings yet. `make fmt-check` fails on `packages/go/internal/ffi/src/lib.rs`:
  binding scaffold writes its `allow` attribute in one line of 125 characters, beyond `max_width = 120` of
  `rustfmt.toml`, and the file is not edited by hand (L-2).

- `Cargo.lock` pins the binding whose generator holds the value aliases, the fault injection and the error units in one
  line of development (CK-20), and the binding packages are those that its scaffold writes: each binding crate takes
  `polyspec-binding-core` with the features `construct` and `lock`, and the lock gains `tokio` as a dependency of
  `polyspec-binding-core`. The contract check has no finding. The generated sources gain a compile-time proof that each
  error variant a member raises is a variant of `Error` (E-5), and the generated Node tests also check each raised error
  with `instanceof` of its class. The behavior of the three packages is the same.

- `make test-php` and `make lint-bindings` find PHP without an argument: the default of `PHP` and of `PHP_CONFIG` is
  the path that `command -v` gives. Before, the default was the bare name `php`, which ext-php-rs reads as a path of a
  file and fails with "php executable not found at php".

- Every tracked document has its Korean translation and the markers of its pair: `AGENTS.ko.md` and
  `docs/spec/cookies.ko.md` are new, each document starts with its `doc-id`, and each Korean file states the sha256 of
  its English file (`source-sha256`). `AGENTS.md` states the rule.

- `composer.lock` is tracked, so `composer install` installs the PHPUnit that runs the tests of the PHP extension at one
  pinned version.

- The bindings build and load on macOS: `.cargo/config.toml` tells the macOS linker to leave the symbols of PHP and
  Node undefined, which the runtime gives when it loads the library, and `make test-node` and `make test-php` name the
  library that Cargo builds by the extension of the platform (`dylib` on macOS, `so` on Linux). Before, the PHP
  extension did not link on macOS and both recipes named a `.so` file that Cargo does not write there.

- `scripts/binding.mts` clones the checkout that Cargo made of the pinned binding commit into `var/binding/<commit>`
  and no longer copies its files. `binding check` reads its own commit from its Git checkout and compares it with the
  commit of `Cargo.lock` (CK-20); with a copy of the files it found no commit, and `make binding-check` failed.

- `Cargo.lock` pins binding at 9de9d03, the commit that every repository of the library family pins (CK-20), and the
  binding packages are those that its scaffold writes: the Node package holds no addon and has one optional dependency
  per platform package. `make test-node` packs the platform package of the machine as a release does and installs both
  packages.
