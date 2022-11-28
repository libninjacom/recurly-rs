<div id="top"></div>

<p align="center">
    <a href="https://github.com/libninjacom/recurly-rs/graphs/contributors">
        <img src="https://img.shields.io/github/contributors/libninjacom/recurly-rs.svg?style=flat-square" alt="GitHub Contributors" />
    </a>
    <a href="https://github.com/libninjacom/recurly-rs/stargazers">
        <img src="https://img.shields.io/github/stars/libninjacom/recurly-rs.svg?style=flat-square" alt="Stars" />
    </a>
    <a href="https://github.com/libninjacom/recurly-rs/actions">
        <img src="https://img.shields.io/github/workflow/status/libninjacom/recurly-rs/CI?style=flat-square" alt="Build Status" />
    </a>
    
<a href="https://crates.io/crates/recurly">
    <img src="https://img.shields.io/crates/d/recurly?style=flat-square" alt="Downloads" />
</a>
<a href="https://crates.io/crates/recurly">
    <img src="https://img.shields.io/crates/v/recurly?style=flat-square" alt="Crates.io" />
</a>

</p>

Recurly client, generated from the OpenAPI spec.

# Usage

```rust
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let response = client
        .list_sites()
        .ids(&["your ids"])
        .limit(1)
        .order("your order")
        .sort("your sort")
        .state("your state")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}

```

This example loads configuration from environment variables, specifically:

* `RECURLY_BASE_URL`

* `RECURLY_API_KEY`



# Installation

Add this to your Cargo.toml:

```toml
[dependencies]
recurly = "48.0.0"
```


# Documentation



* [Client Library Documentation](https://docs.rs/recurly)


You can see working examples of every API call in the `examples/` directory.

# Contributing

Contributions are welcome!

*Library created with [Libninja](https://www.libninja.com).*
