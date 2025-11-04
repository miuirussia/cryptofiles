# Project Overview

This project is a command-line utility written in Rust for encrypting and decrypting the contents of a folder. It encrypts all files and subdirectories, including their names, and saves the encryption key to a file named `key.txt` in the original folder.

The project uses the `magic-crypt` crate for encryption, `rand` for generating a random encryption key, and `base64` for encoding.

## Key Files

-   `src/main.rs`: The main entry point for the command-line application. It handles argument parsing and calls the appropriate encryption or decryption functions.
-   `src/encrypt.rs`: Contains the logic for encrypting a folder's contents and file paths.
-   `src/decrypt.rs`: Contains the logic for decrypting a folder's contents and file paths.
-   `Cargo.toml`: The project's manifest file, which defines the dependencies and project metadata.
-   `build.sh`: A script to build the project into a WebAssembly (WASM) binary.

# Building and Running

## Building

To build the project for your native platform, run:

```bash
cargo build
```

To build the project as a WebAssembly binary, run:

```bash
./build.sh
```

## Running

The application is run from the command line with the following syntax:

### Encryption

```bash
cargo run -- encrypt <folder_path>
```

This will encrypt the specified folder and create a `key.txt` file inside the original folder.

### Decryption

```bash
cargo run -- decrypt <encrypted_folder_path> <key> <destination_path>
```

This will decrypt the specified folder into the destination path using the provided key.

## Testing

To run the tests, use the following command:

```bash
cargo test
```

# Development Conventions

The project follows the standard Rust project structure. The code is organized into modules for encryption and decryption. Tests are included in the `src/main.rs` file within a `#[cfg(test)]` module.
