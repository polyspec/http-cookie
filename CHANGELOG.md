<!-- doc-id: changelog -->

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
