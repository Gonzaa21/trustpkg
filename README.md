# Trustpkg - Dependency Analyzer
> A Rust CLI that helps developers evaluate the trustworthiness of third-party dependencies before using them.

## Functionality
Trustpkg analyzes third-party dependencies by combining security, maintenance, reputation, and ecosystem data into a single report, helping developers make informed decisions before adding a package to their projects.

## Features

- Analyze Rust crates
- Unified trust report
- Security insights
- Maintenance metrics
- Community health
- Explainable Trust Score

## Usage Example
```bash
cargo run -- analyze <crate> --version <version> -- format <format> (json or text)
```
```bash
cargo run -- analyze emit_ansi_term

== 1. PACKAGE DETAILS ==
Name:        emit_ansi_term v0.2.0
Description: An ANSI (colored) terminal collector for emit.
Repository:  N/A
License:     Apache-2.0
Created:     2016-06-23 09:31:43.452328 UTC
Updated:     2016-07-05 01:01:25.610080 UTC
Is yanked:   false
Downloads:   4789 (recent: 8)
Sources:     crates.io, osv.dev

== 2. SCORE ==
Maintenance  [######----] Unknown
Popularity   [####------] 46
Security     [##########] 100 | 0 advisory (0 critical, 0 high)
----------------------------------------
TRUST SCORE  [#######---] 77 / 100

VERDICT      REVIEW

== 3. FINAL REPORT ==
[INFO] Maintenance could not be verified (no GitHub repository or unreachable)
```

## Installation

### Prerequisites
- check if this is needed

### Installation via npm / cargo
```bash
# to be added later
```

## Contributing
Please consult the [contributing guide](CONTRIBUTING.md) to learn how to report bugs, propose new features, or submit pull requests.

## License
This project is licensed under the [MIT](LICENSE) license.