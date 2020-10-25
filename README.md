cargo-geiger ☢️ 
===============

[![Build Status](https://dev.azure.com/cargo-geiger/cargo-geiger/_apis/build/status/rust-secure-code.cargo-geiger?branchName=master)](https://dev.azure.com/cargo-geiger/cargo-geiger/_build/latest?definitionId=1&branchName=master)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![Code Coverage](https://img.shields.io/azure-devops/coverage/cargo-geiger/cargo-geiger/2/master)](https://img.shields.io/azure-devops/coverage/cargo-geiger/cargo-geiger/2/master)
[![crates.io](https://img.shields.io/crates/v/cargo-geiger.svg)](https://crates.io/crates/cargo-geiger)
[![Crates.io](https://img.shields.io/crates/d/cargo-geiger?label=cargo%20installs)](https://crates.io/crates/cargo-geiger)

A program that list statistics related to usage of unsafe Rust code in a Rust
crate and all its dependencies.

This cargo plugin is based on the code from two other projects:
<https://github.com/icefoxen/cargo-osha> and
<https://github.com/sfackler/cargo-tree>.

Installation
------------

Try to find and use a system-wide installed OpenSSL library:
```
cargo install cargo-geiger
```

Or, build and statically link OpenSSL as part of the cargo-geiger executable:
```
cargo install cargo-geiger --features vendored-openssl
```

Usage
-----

1. Navigate to the same directory as the `Cargo.toml` you want to analyze.
2. `cargo geiger`


Output example
--------------

![Example output](https://user-images.githubusercontent.com/3704611/53132247-845f7080-356f-11e9-9c76-a9498d4a744b.png)


Why even care about unsafe Rust usage?
--------------------------------------

When and why to use unsafe Rust is out of scope for this project, it is simply
a tool that provides information to aid auditing and hopefully to guide
dependency selection. It is however the opinion of the author of this project
that __libraries choosing to abstain from unsafe Rust usage when possible should
be promoted__.

This project is an attempt to create pressure against __unnecessary__ usage of
unsafe Rust in public Rust libraries.


Why the name?
-------------

<https://en.wikipedia.org/wiki/Geiger_counter>

Unsafe code and ionizing radiation have something in common, they are both
inevitable in some situations and both should preferably be safely contained!


Known issues
------------

 - Unsafe code inside macros are not detected. Needs macro expansion(?).
 - Unsafe code generated by `build.rs` are probably not detected.
 - More on the github issue tracker.


Roadmap
-------

 - ~~There should be no false negatives. All unsafe code should be
   identified.~~ This is probably too ambitious, but scanning for
   `#![forbid(unsafe_code)]` should be a reliable alternative (implemented since
   0.6.0). Please see the changelog.
 - An optional whitelist file at the root crate level to specify crates that are
   trusted to use unsafe (should only have an effect if placed in the root
   project).


Changelog
---------

### 0.11.0
 - TODO: Prepare release.

### 0.10.2
 - __Bugfix__: Avoid panic and log warnings on parse failure. [#105]
 - Upgraded all dependencies.

### 0.10.1
 - Expose the `cargo` crate feature: `vendored-openssl`. [#99]
 - Upgraded all dependencies.

### 0.10.0
 - Upgraded all dependencies. [#98]

### 0.9.1
 - __Bugfix__: Avoid counting the same crate multiple times. [#79]
 - Upgraded cargo to 0.41. [#85]
 - Upgraded all dependencies.

### 0.9.0
 - __Breaking change__: Replaced structopt & clap with [pico-args], to reduce 
   compile times [#77]. As a result the `-Z` flag now requires quotes around
   its list of sub arguments, other than that there should be no changes to 
   the CLI.

### 0.8.0
 - __Bugfix:__ Count all expressions in unsafe functions and nested unsafe
   scopes, in [geiger 0.4.1](geiger), [#72] & [#71].
 - __Bugfix:__ Properly account for possibly patched dependencies [#70].
 - Summary for each metrics column, [#76].
 - Now requires all entry points for a crate to declare
   `#[forbid(unsafe_code)]` for it to count as crate-wide.
 - New optional scan mode `--forbid-only`. This mode doesn't require any calls
   to `rustc` and only requires parsing the entry point `.rs` files, making it
   much faster than the normal mode.
 - Updated dependencies.

### 0.7.3
 - __Bugfix:__ Fix dependency collection for mixed workspaces [#66].
 - Updated dependencies.

### 0.7.2
 - Updated dependencies to fix [#59].

### 0.7.1
 - __Bugfix:__ related to attributes, in [geiger] [#57].
 - Updated all dependencies.

### 0.7.0
 - Updated all dependencies, [geiger] to 0.3.0.

### 0.6.1
 - A tiny readme fix.

### 0.6.0
 - There are now three crate scanning result variants [#52]:
   - 🔒 No unsafe usage found and all build target entry point `.rs` source
     files, used by the build, declare `#![forbid(unsafe_code)]`. Crates like
     this will be printed in green.
   - ❓ No unsafe usage found, but at least one build target entry point `.rs`
     file, used by the build, does not declare `#[forbid(unsafe_code)]`.  Crates
     like this will be printed in the default terminal foreground color.
   - ☢️  Unsafe usage found. Crates like this will be printed in red, same as in
     the previous version.

### 0.5.0
 - Moved resusable parts, decoupled from `cargo`, to the new crate
   [geiger]. Main github issue: [#30].
 - Some general refactoring and cleanup.
 - Merge pull request [#46] from alexmaco/dependency_kind_control. add options
   to filter dependencies by kind; defaults to Kind::Normal.
 - Merge pull request [#40] from jiminhsieh/rust-2018. Use Rust 2018 edition.

### 0.4.2
 - __Bugfix:__ Merge pull request [#33] from ajpaverd/windows_filepaths.
   Canonicalize file paths from walker.

 - Merge pull request [#38] from anderejd/updated-deps. Updated deps and fixed
   build errors.

### 0.4.1
 - Merge pull request [#28] from alexmaco/deps_upgrade. fix build on rust 1.30:
   upgrade petgraph to 0.4.13

 - __Bugfix:__ Merge pull request [#29] from alexmaco/invalid_utf8_source. fix 
   handling source files with invalid utf8: lossy conversion to string

### 0.4.0
 - Filters out tests by default. Tests can still be included by using
   `--include-tests`. The test code is filted out by looking for the attribute
   `#[test]` on functions and `#[cfg(test)]` on modules.

### 0.3.1
 - __Bugfix:__ Some bugfixes related to cargo workspace path handling.
 - Slightly better error messages in some cases.

### 0.3.0
 - Intercepts `rustc` calls and reads the `.d` files generated by `rustc` to
   identify which `.rs` files are used by the build. This allows a crate that
   contains `.rs` files with unsafe code usage to pass as "green" if the unsafe
   code isn't used by the build.
 - Each metric is now printed as `x/y`, where `x` is the unsafe code used by the
   build and `y` is the total unsafe usage found in the crate.
 - Removed the `--compact` output format to avoid some code complexity. A new
   and better compact mode can be added later if requested.

### 0.2.0
 - Table based output format [#9].

### 0.1.x
 - Initial experimental versions.
 - Mostly README.md updates.

[#9]: https://github.com/rust-secure-code/cargo-geiger/pull/9
[#28]: https://github.com/rust-secure-code/cargo-geiger/issues/28
[#29]: https://github.com/rust-secure-code/cargo-geiger/issues/29
[#30]: https://github.com/rust-secure-code/cargo-geiger/issues/30
[#33]: https://github.com/rust-secure-code/cargo-geiger/issues/33
[#38]: https://github.com/rust-secure-code/cargo-geiger/issues/38
[#40]: https://github.com/rust-secure-code/cargo-geiger/issues/40
[#46]: https://github.com/rust-secure-code/cargo-geiger/issues/46
[#52]: https://github.com/rust-secure-code/cargo-geiger/issues/52
[#57]: https://github.com/rust-secure-code/cargo-geiger/issues/57
[#59]: https://github.com/rust-secure-code/cargo-geiger/issues/59
[#66]: https://github.com/rust-secure-code/cargo-geiger/issues/66
[#70]: https://github.com/rust-secure-code/cargo-geiger/pull/70
[#71]: https://github.com/rust-secure-code/cargo-geiger/issues/71
[#72]: https://github.com/rust-secure-code/cargo-geiger/pull/72
[#76]: https://github.com/rust-secure-code/cargo-geiger/pull/76
[#77]: https://github.com/rust-secure-code/cargo-geiger/pull/77
[#79]: https://github.com/rust-secure-code/cargo-geiger/issues/79
[#85]: https://github.com/rust-secure-code/cargo-geiger/pull/85
[#98]: https://github.com/rust-secure-code/cargo-geiger/pull/98
[#99]: https://github.com/rust-secure-code/cargo-geiger/pull/99
[#105]: https://github.com/rust-secure-code/cargo-geiger/issues/105
[geiger]: https://crates.io/crates/geiger
[pico-args]: https://crates.io/crates/pico-args

