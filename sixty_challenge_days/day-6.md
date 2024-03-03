# Day 6: Beyond the basic of Linked List - 25-02-2024

## Scenario: A Simple File Format
Building a basic config/setting file parser. The format will be intentionally simplified:
- Lines: Settings are represented as ``key = value`` on separate lines.
- Types: For now, let's just support strings, integers and boolean.
- Example: ```username = "Fellippe"```.

## Tasks:
1. Struct to represent a single setting.
2. Struct to represent the entire config file.
3. Write a function that takes a string slice (representing a single line of the file) and tries to extract a setting from it.
4. Return an error if the format is incorrect.

## Rust Concepts in Play
- Structs & Enums: Core to representing your data.
- Error Handling: Likely using the Result type to signal parsing issues.
- File I/O: If you tackle this part, you'll get to use Rust's standard library for file operations.

#### Choose Your Path
I want to get deeper in both paths
