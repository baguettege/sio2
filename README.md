# sio2

A deterministic, zero-copy binary serialization library with optional `alloc`
and `std` support.

---

## Features

- deterministic encoding API for both allocating and non-allocating
- zero-copy decoding API
- proc macro derives
- full `#[no_std]` support

---

## Usage

### Encoding

Using the allocating API:

```rust
use sio2::{VecExt, ToBytes};

fn main() {
    let bytes: Vec<u8> = "silica".to_bytes();
    
    let mut buf = Vec::new();
    buf.encode("silica2");
    buf.encode(&321u8);
}
```

Using the non-allocating API:

```rust
use sio2::{BufMut, EncodeResult};

fn main() -> EncodeResult<()> {
    let mut bytes = [0u8; 8];
    let mut buf = BufMut::new(&mut bytes);
    
    buf.encode(&123u32)?;
    buf.encode(&321u32)?;
    
    Ok(())
}
```

### Decoding

```rust
use sio2::{Buf, DecodeResult};

fn main() -> DecodeResult<()> {
    let bytes = [0u8, 0u8, 1u8, 0u8];
    let mut buf = Buf::new(&bytes);
    
    let n1: u16 = buf.decode()?; // Should be `0`
    let n2: u16 = buf.decode()?; // Should be `256`
    
    Ok(())
}
```

### Deriving

All `Encode` implementations automatically implement the `ToBytes` extension trait.

```rust
use sio2::{Encode, Decode};
use std::borrow::Cow;

// Simple struct
#[derive(Encode, Decode)]
struct Point {
    x: i32,
    y: i32,
}

// Complex enum with borrowing from the input buffer
#[derive(Encode, Decode)]
#[sio2(borrow('buf))]
enum Complex<'buf> { 
    Named { bytes: &'buf [u8] }, 
    Unnamed(Cow<'buf, str>), 
    Unit,
}
```

---

## Feature Flags

| Feature     | Description                                                    |
|-------------|----------------------------------------------------------------|
| `alloc`     | enables `alloc` types and the allocating `Encode` API          |
| `std`       | enables `std` types and `alloc`                                |
| `indexmap`  | enables `indexmap`'s `IndexMap` and `IndexSet` implementations |
| `hashbrown` | enables `hashbrown`'s `HashMap` and `HashSet` implementations  |

---

## Endian-ness

All implementations for numbers use big-endian.

---

## License

This project is licensed under the MIT License.
