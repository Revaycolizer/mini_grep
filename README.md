# mini_grep

A lightweight command-line text search utility written in Rust. It searches for a given pattern within a file and prints matching lines with colored highlights.

## Features

- Case-insensitive search
- Line number display
- Match count mode
- Inverted matching (print lines that do not match)
- Colored output with matched text highlighted in bold red
- Regex-safe pattern escaping

## Requirements

- Rust 2024 edition (rustc 1.85+)

## Installation

```
git clone https://github.com/revaycolizer/mini_grep.git
cd mini_grep
cargo build --release
```

The compiled binary will be located at `target/release/mini_grep`.

## Usage

```
mini_grep [OPTIONS] <NEEDLE> <HAYSTACK>
```

### Arguments

| Argument   | Description                          |
|------------|--------------------------------------|
| `NEEDLE`   | The pattern to search for            |
| `HAYSTACK` | Path to the file to search within    |

### Options

| Flag                   | Description                                      |
|------------------------|--------------------------------------------------|
| `-i`, `--ignore-case`  | Perform a case-insensitive search                |
| `-n`, `--line-numbers` | Prefix each matching line with its line number   |
| `-c`, `--count-matches`| Print only the count of matching lines           |
| `-v`, `--invert-match` | Print lines that do not match the pattern        |
| `-h`, `--help`         | Print help information                           |

### Examples

Search for "error" in a log file:

```
mini_grep error server.log
```

Case-insensitive search with line numbers:

```
mini_grep -i -n "warning" app.log
```

Count the number of matching lines:

```
mini_grep -c "TODO" main.rs
```

Print all lines that do not contain "debug":

```
mini_grep -v "debug" output.log
```

## Project Structure

```
src/
  main.rs                  -- CLI argument parsing and entry point
  tools/
    mod.rs                 -- Module declarations
    grep.rs                -- File reading and delegation to search
    search_in_file.rs      -- Core search logic, regex matching, and output formatting
```

## License

This project is provided as-is for personal and educational use.
