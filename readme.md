# Rest Client

## Install

In your Cargo.toml:

```toml
[dependencies]
"rest-client" = "0.1"
reqwest = "0.9"

[dependencies.serde]
version = "1.0"
features = [
    "derive"
]
```

## Usage

```rust
extern crate reqwest;
extern crate rest_client;
use rest_client::*;
use serde::Deserialize;

#[rest("https://example.com/rest-api/{}/multiple?variables={}")]
#[rest("https://example.com/{}", vec)] // if it returns multiple elements
#[derive(Deserialize)]
struct Foo {
    hello: String
}

fn main() {
    let foo = Foo::get(vec!["my", "arguments"]).unwrap(); // Box<Foo>
    let bar = Foo::gets(vec![42]).unwrap(); // Vec<Box<Foo>>
}
```

## License

This project is licensed under either of

* [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
  ([LICENSE-APACHE](LICENSE-APACHE))

* [MIT License](http://opensource.org/licenses/MIT)
  ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Contributing

I am happy about every contribution to this project.

Contributions in any form (issues, pull requests, etc.) to this project
must adhere to Rust's [Code of Conduct].

[Code of Conduct]: https://www.rust-lang.org/en-US/conduct.html
