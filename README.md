# gitp-ush

A small demo that automatically corrects git typos and simplifies git push with automatic upstream setting.

## Features

- **Typo Autocorrection**: Automatically detects and corrects common git push typos:
  - `gitp ush` → `git push`
  - `git psuh` → `git push`
  - `git puhs` → `git push`
  - `git pus` → `git push`
  - `gi tpush` → `git push`
  - `gti push` → `git push`

- **Automatic Upstream**: Automatically adds `-u origin <current-branch>` to git push commands

## Installation

```bash
cargo build --release
```

Optional: Add to your PATH or create an alias:

```bash
# Add alias to your shell config (.bashrc, .zshrc, etc.)
alias gitp='cargo run --quiet --'
```

## Usage

### Basic usage:

```bash
# Using cargo run
cargo run -- gitp ush

# Or if you built the release binary
./target/release/gitp-ush gitp ush
```

### What happens:

```
🔧 Autocorrecting: 'gitp ush' -> 'git push'
📍 Current branch: main
🚀 Executing: git push -u origin main
✅ Successfully pushed to origin/main
```

### More examples:

```bash
# All of these work the same way:
cargo run -- gitp ush
cargo run -- git psuh
cargo run -- git push
cargo run -- gti push

# With additional arguments:
cargo run -- gitp ush --force
cargo run -- git push --tags
```

## How it works

1. Detects common typos in the input
2. Automatically corrects them
3. Gets the current git branch
4. Executes `git push -u origin <branch>` with any additional arguments
5. Reports success or failure

## Demo Purpose

This is a demonstration project showing how to build a CLI tool that:
- Parses command-line arguments
- Detects and corrects typos
- Interacts with git commands
- Provides user-friendly output
