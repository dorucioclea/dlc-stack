# dlc-storage

The `dlc-storage` project is a Rust framework for providing storage operations for DLC (downloadable content) related components.

## Installation

To install `dlc-storage`, you can add it as a dependency to your `Cargo.toml` file:

```ini
[dependencies]
dlc-storage = "0.1.0"
```

Then, you can run `cargo build` to download and build the crate.

## Usage

To use `dlc-storage` in your Rust project, you will need to import it and create an instance of the `Storage` struct:

```rust
extern crate dlc-storage;

use dlc-storage::Storage;

let storage = Storage::new();
```

You can then use the `storage` instance to perform storage operations, such as saving and retrieving data from the storage backend.

## Dependencies

The `dlc-storage` project has the following dependencies:

- [Rust](https://www.rust-lang.org/) >= 1.45.0
- [Cargo](https://doc.rust-lang.org/cargo/) >= 1.0.0

## License

The `dlc-storage` project is licensed under the [MIT license](LICENSE).
