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

Using with encoding:

```rust
extern crate chardet;
extern crate encoding;
use chardet;
use std::fs::OpenOptions;
use std::io::prelude::*;
use encoding::DecoderTrap;
use encoding::label::encoding_from_whatwg_label;

// open text file
let mut fh = OpenOptions::new().read(true).open(filepath).expect(
    "Could not open file",
);
let mut reader: Vec<u8> = Vec::new();

// read file
fh.read_to_end(&mut reader).expect("Could not read file");

// detect charset of the file
let result = detect(&reader);
// result.0 Encode
// result.1 Confidence
// result.2 Language

// decode file into utf-8
let coder = encoding_from_whatwg_label(charset2encoding(&result.0));
if coder.is_some() {
    let utf8reader = coder.unwrap().decode(&reader, DecoderTrap::Ignore).expect("Error");
}
```

