Rust-Chardet
=========================
[![Rust-Charset on Travis CI][travis-image]][travis]

[travis-image]: https://travis-ci.org/thuleqaid/rust-chardet.png
[travis]: https://travis-ci.org/thuleqaid/rust-chardet

Rust version of chardet.

## Usage

Put this in your `Cargo.toml`:

```toml
[dependencies]
chardet = "0.2"
```

Then put this in your crate root:

```rust
extern crate chardet;
```

To detect charset:

```rust
use chardet;

let result_file:(String, f32, String) = chardet::detect_file("FilePath");
// result_file.0 Encode
// result_file.1 Confidence
// result_file.2 Language

let mut bindata:Vec<u8> = Vec::new();
// load file/data into bindata
let result_vec:(String, f32, String) = chardet::detect(&bindata);
// result_vec.0 Encode
// result_vec.1 Confidence
// result_vec.2 Language
```

