
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
