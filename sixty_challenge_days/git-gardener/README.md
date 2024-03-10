# Git Gardener 🌿🧑🏽‍🌾

A Rust-powered command-line tool to help you keep your Git repositories tidy by pruning stale and merged branches.

## Key Features

* **Staleness Detection:** Identifies branches that haven't been updated within a specified timeframe (customizable, defaults to 3 months).
* **Merged Branch Cleanup:** Finds branches whose history is fully contained within your main branch.
* **Dry-Run Mode:** Preview which branches would be deleted, without actually performing the deletion.
* **Flexible Configuration:** Override the target main branch and adjust the staleness threshold.

## Installation

**Prerequisites:**
* Rust toolchain (https://www.rust-lang.org/tools/install)
* Git

**Using Cargo:**
```bash
cargo install git-gardener

# Basic cleanup with default settings
git-gardener

# Preview mode (no deletions)
git-gardener --dry-run

# Override the main branch to 'develop' and set staleness to 6 months
git-gardener --main-branch develop --staleness 6months 

# Get help
git-gardener --help
```

## Contributing

This project is open to contributions! Feel free to open issues, suggest features, or submit a pull request.
License


[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](LICENSE-APACHE)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE-MIT)

Dual-licensed under [Apache 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT).
