# Leo Chen Mini 7

[![CI](https://github.com/nogibjj/Leo_Chen_Mini_7/actions/workflows/CI.yml/badge.svg)](https://github.com/nogibjj/Leo_Chen_Mini_7/actions/workflows/CI.yml)

## Echo - A Simple CLI Tool

`echo` is a simple command-line tool built in Rust using the Clap crate. The tool accepts a single argument and prints it back to the user. It is designed to showcase basic CLI functionalities and is suitable for further development into more complex projects.

## Rust Project Folder File Structure
```
echo/
├── src/
│   └── main.rs         # Source code for the CLI tool
├── target/             # Contains compiled binaries (generated after build), not uploaded to GitHub
├── Cargo.toml          # Project dependencies and metadata
├── Cargo.lock
├── Makefile
```

## Installation

### Prerequisites
- **Rust** and **Cargo** installed on your machine. [Installation Guide](https://www.rust-lang.org/tools/install)

### Build the Project
```
cargo build --release
```

### Option 1: Temporary
Add the compiled binary to your `PATH`:
```
export PATH=$PATH:.../echo/target/release
```

### Option 2: Install Globally
```
cargo install --path .
```

## Usage
Run the tool with a single argument:
```
echo <argument>
```
For example:
```
echo Hello
```
Output:
```
You entered: Hello
```

Use `--help` to see more information.