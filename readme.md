# Rest Client

## Install

In your Cargo.toml:

```toml
[dependencies]
"rest-client" = "0.1"
reqwest = "0.9"
```

## Usage

```rust
extern crate reqwest;
extern crate rest_client;
use rest_client::*;
use serde::Deserialize;

[rest("https://example.com/rest-api/{}/multiple?variables={}")]
[rest("https://example.com/{}", vec)] // if it returns multiple elements
#[derive(Deserialize)]
struct Foo {
    hello: String
}

fn main() {
    let foo = Foo::get(vec!["my", "arguments"]).unwrap(); // Box<Foo>
    let bar = Foo::gets(vec![42]).unwrap(); // Vec<Box<Foo>>
}
```
