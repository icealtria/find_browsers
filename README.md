# Your Library Name

A library for retrieving installed browsers (supports Windows and Linux).

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
find_browsers = "0.1.0"
```

## Usage
```rust
use find_browsers::get_browsers;

fn main() {
    let browsers = get_browsers().unwrap();
    for browser in browsers {
        println!("Found browser: {} at {:?}", browser.name, browser.exec);
    }
}
```