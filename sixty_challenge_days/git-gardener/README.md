<p align="center">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://github.com/lucaspere/rust_projects/assets/46873546/0cb6ec24-93c9-46c2-a70c-d97b9b215df7">
      <source media="(prefers-color-scheme: light)" srcset="https://github.com/lucaspere/rust_projects/assets/46873546/0cb6ec24-93c9-46c2-a70c-d97b9b215df7">
      <img alt="Git-Gardener" title="Ferris Gardener" src="https://github.com/lucaspere/rust_projects/assets/46873546/0cb6ec24-93c9-46c2-a70c-d97b9b215df7" height="350px">
    </picture>
</p>



Git Gardener 🌿🧑🏽‍🌾 is a Rust-powered command-line tool to help you keep your Git repositories tidy by pruning stale and merged branches.

## Key Features

* **Staleness Detection:** Identifies branches that haven't been updated within a specified timeframe (customizable, defaults to 3 months).
* **Merged Branch Cleanup:** Finds branches whose history is fully contained within your main branch.
* **Dry-Run Mode:** Preview which branches would be deleted, without actually performing the deletion.
* **Flexible Configuration:** Override the target main branch and adjust the staleness threshold.

## Roadmap (Upcoming Enhancements):
*   **Core Functionality**
      *   ~~**Stale Branch Cleanup:** Robust deletion of branches based on a configurable staleness threshold (e.g., no commits in the past 3 months).~~
      *   ~~**Merged Branch Cleanup:**  Identify and delete branches whose history is fully contained within the main/target branch.~~
      *   ~~**Dry Run Mode:** Preview the actions that would be taken without actually deleting branches.~~
      *   ~~**User-Friendly Output:** Clear summaries of branches targeted for deletion, and success/error messages.~~
      *   **Interactive Mode:** Prompt the user for confirmation before deleting each branch or allow bulk selection of branches for deletion.
      *   **Forced Deletion:** A `--force` flag to override staleness and merged branch criteria, allowing the deletion of any specified branch (use with caution!).
      *   **Branch Filtering:** Options to include/exclude branches based on name patterns (e.g., `--exclude 'release/*'`).

*   **Gardener Mode & Refinements**
      *   **Background Execution:**  Implement the "gardener" mode to periodically trigger cleanups on a schedule (using `cron` or similar).
      *   **Configuration File:**  Allow users to store their preferred settings (main branch, default staleness, etc.) in a `.gitgardener` config file.
      *   **Customizable Output:**  Options to format the output (text, simple JSON) for integration into other scripts or tools.
 
*   **Robustness & User Experience**

      *   **Comprehensive Testing**: Test suite covering various branch scenarios (staleness, merged status, edge cases) to ensure reliability.
      *   **Informative Error Handling**: Catch Git interaction errors, provide meaningful messages to the user, and handle unexpected input gracefully.
      *   **Main Branch Override**: Allow cleanup against branches other than "main".
      *   **Interactive Confirmation**: Prompt the user to confirm deletions before proceeding, enhancing safety..

*   **Phase 4: Reusability & Distribution**
      *   **Code Refactoring:**  Modularize logic for potential use as a library within other Rust projects.
      *   **Explore crates.io Publishing:**  Consider packaging `git-gardener` for easy installation via `cargo install git-gardener`.




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
