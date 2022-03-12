# Japhonex Rust

Japanese phone number checker for Rust.

## Installation
```toml
[dependencies]
japhonex = "0.1"
```

## Usage
In your Go app you can do something like:
```rust
extern crate japhonex;
use japhonex::japhonex;
use japhonex::HyphenCheck;

fn main () {
    let r = japhonex(HyphenCheck::Optional);

    r.is_match("<your input>");
}
```

### Hyphen validation patterns
### Optional
```rust
let r = japhonex.OptionalHyphen(HyphenCkeck::Optional);
// 0xx-xxxx-xxxx or 0xxxxxxxxxx
```
### Required
```rust
let r = japhonex.OptionalHyphen(HyphenCkeck::Required);
// 0xx-xxxx-xxxx
```
### No hyphen
```rust
let r = japhonex.OptionalHyphen(HyphenCkeck::NoCheck);
// 0xxxxxxxxxx
```

## Licence
MIT