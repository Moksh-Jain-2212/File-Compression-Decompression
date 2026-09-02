# File Compression-Decompression

A simple Rust project that demonstrates file compression and decompression using gzip.

The program reads an input file, compresses it into a `.gz` file, then reads the compressed file and decompresses it back into a normal output file.

## Features

- Compresses files using gzip compression
- Decompresses `.gz` files
- Uses buffered file reading for efficient file handling
- Built with Rust and the `flate2` crate

## Requirements

- Rust
- Cargo

Check that Rust is installed:

```bash
rustc --version
cargo --version
```

## Installation

Clone the repository:

```bash
git clone <your-repository-url>
cd File\ Compression-Decompression
```

Install dependencies and check the project:

```bash
cargo check
```

## Usage

Before running the project, update the file paths in `src/main.rs`.

Replace the placeholder values:

```rust
File::open("File Path")
File::create("File Path")
```

with real input and output paths.

Run the project:

```bash
cargo run
```

Expected output:

```text
Compression complete
Decompression complete
```

## Project Structure

```text
.
├── Cargo.toml
├── Cargo.lock
├── README.md
└── src
    └── main.rs
```

## Dependency

This project uses:

- `flate2` for gzip compression and decompression

## Notes

The current code is written as a simple learning/demo project. For regular use, the file paths can be improved by accepting command-line arguments instead of hard-coded paths.
