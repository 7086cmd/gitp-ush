# gitp-ush

Write `gitp ush` and execute `git push -u origin <current-branch>`.

A small demo that turns your typo into a feature - making `gitp ush` a shortcut for `git push` with automatic upstream setting.

## Features

- Simple typo-to-feature conversion: `gitp ush` → `git push -u origin <branch>`
- Automatically detects your current git branch
- Automatically sets upstream tracking
- Supports additional git push arguments

## Installation

```bash
cargo build --release
```

The binary will be created at `./target/release/gitp`

Optional: Add to your PATH:

```bash
# Copy to a directory in your PATH
sudo cp ./target/release/gitp /usr/local/bin/

# Or add an alias to your shell config (.bashrc, .zshrc, etc.)
alias gitp='/path/to/target/release/gitp'
```

## Usage

### Basic usage:

```bash
gitp ush
```

### What happens:

```
Current branch: main
Executing: git push -u origin main
Successfully pushed to origin/main
```

### With additional arguments:

```bash
gitp ush --force
gitp ush --tags
gitp ush --force-with-lease
```

## How it works

1. You run `gitp ush` (either by typo or intentionally)
2. The tool detects the current git branch
3. Executes `git push -u origin <branch>` with any additional arguments
4. Sets upstream tracking automatically

## Demo Purpose

This demonstrates how to build a simple CLI tool in Rust that:
- Parses command-line arguments
- Executes git commands programmatically
- Provides a better UX by automating common tasks
- Turns a common typo into a useful feature
