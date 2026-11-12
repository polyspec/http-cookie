
- `Cargo.lock` pins binding at 9de9d03, the commit that every repository of the library family pins (CK-20), and the
  binding packages are those that its scaffold writes: the Node package holds no addon and has one optional dependency
  per platform package. `make test-node` packs the platform package of the machine as a release does and installs both
  packages.
