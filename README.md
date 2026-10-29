# Trustpkg - Dependency Analyzer
> A Rust CLI that helps developers evaluate the trustworthiness of third-party dependencies before using them.

## Functionality
Trustpkg analyzes third-party dependencies by combining security, maintenance, reputation, and ecosystem data into a single report, helping developers make informed decisions before adding a package to their projects.

## Features

- Analyze Rust crates from crates.io
- Unified trust report, in terminal or JSON format
- Security insights: known advisories from [OSV.dev](https://osv.dev) (includes RustSec)
- Maintenance metrics: repository activity from GitHub
- Popularity metrics: download counts
- Explainable Trust Score with a final verdict

## Usage

```bash
trustpkg analyze <crate> [--version <version>] [--format <text|json>]
```

| Option | Description | Default |
|---|---|---|
| `<crate>` | Name of the crate to analyze | required |
| `--version` | Specific version to analyze | latest |
| `--format` | Output format: `text` or `json` | `text` |

While developing, replace `trustpkg` with `cargo run --`.

### Example

```bash
trustpkg analyze ansi_term
```

```
== 1. PACKAGE DETAILS ==
Name:        ansi_term v0.12.1
Description: Library for ANSI terminal colours and styles (bold, underline)
Repository:  https://github.com/ogham/rust-ansi-term
License:     MIT
Created:     2014-11-26 00:50:42.101172 UTC
Updated:     2019-09-02 10:16:04.908577 UTC
Is yanked:   false
Downloads:   232574994 (recent: 19443837)
Sources:     crates.io, osv.dev

== 2. SCORE ==
Maintenance  [####------] Inactive
Popularity   [##########] 100
Security     [#########-]  95 | 1 advisory (0 critical, 0 high)
----------------------------------------
TRUST SCORE  [######----] 60 / 100 (capped: inactive repository)

VERDICT      REVIEW

== 3. FINAL REPORT ==
[WARN] No repository activity in over a year (Date: 2024-08-01)
[WARN] 1 known advisories (0 critical, 0 high): RUSTSEC-2021-0139 (Count: 1)
```

## How the score works

The Trust Score (0-100) combines three metrics:

| Metric | Weight | How it is computed |
|---|---|---|
| Security | 50% | Starts at 100; -40 per critical advisory, -20 per high, -5 per other known advisory |
| Maintenance | 30% | Based on the last push to the repository (active, inactive or unknown) |
| Popularity | 20% | Logarithmic scale of total downloads |

Severe findings put a **cap** on the overall score (the lowest applicable cap wins):

| Finding | Maximum score |
|---|---|
| Yanked version | 20 |
| Critical advisory | 30 |
| Archived repository | 40 |
| Inactive repository | 60 |

The **verdict** comes from the final score: 80 or more is `TRUSTED`, 50 to 79 is `REVIEW`, and below 50 is `NOT RECOMMENDED`.

If a data source fails (for example, OSV.dev is unreachable), the score is computed with the remaining metrics and marked as `(Partial)`.

## Limitations

- Only crates from crates.io are supported for now.
- Maintenance data comes from the GitHub API, which allows 60 unauthenticated requests per hour. Crates without a GitHub repository get an `Unknown` maintenance status.

## Installation

### Install from source
```bash
git clone https://github.com/Gonzaa21/trustpkg.git
cd trustpkg
cargo install --path .
```

## License
This project is licensed under the [MIT](LICENSE) license.