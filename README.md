# Fdlibm-rs
This crate is a rust wrapper of [fdlibm](https://netlib.org/fdlibm/). 

# Usage
Run the following Cargo command in your project directory:
```bash
cargo add fdlibm-rs
```

```rust
use fdlibm_rs::sin;
let value = unsafe {sin(1.0)};
println!("sin(1)={}",value);
```

# Disclaimer 
1) This lib is currently in beta version, some of functions doesn't have unit tests and some are buggy ([`gamma`](tests/fdlibm.rs#L146)).
2) Other modes decribed in [fdlibm](fdlibm/readme) are not supported yet.

# Todo
1) <s>Avoid using `std::`, instead use `core::` when possible.</s>
2) Support other targets (currently only `x86_64-unknown-linux-gnu` is supported)
3) <s>Support other [Modes](fdlibm/readme).</s>
4) Benchmarks.