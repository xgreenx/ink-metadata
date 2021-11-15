This repository shows how ink! can implement aggregation of metadata from crates. 

On startup of the metadata binary(created by `cargo-contract`) `on_startup` 
macro will init the static variable of metadata defined in 
ink!(it also works from `rlib` crates). 

After ink! can process this variable to generate an ABI. 
This schema allows defining events independently
(And you also can use the same idea to generate ABI for all methods defined by the user).

You should build it with enabled `lto`(it is enabled in profile section of `hello_cargo/Cargo.toml`).
To test this example you need to run:
```shell
cd hello_cargo
cargo build && ./target/debug/hello_cargo
```

You should see the next output:
```shell
Version: 3
selector: 131313, name: psp22_balance_of <------ method from contracts crate
selector: 161616, name: psp721_balance_of <------ method from contracts crate
selector: 1111111, name: some_ink_method
selector: 0, name: fallback_method
selector: 9, name: value
selector: 141414, name: psp22_transfer <------ method from contracts crate
selector: 8, name: flip
selector: 151515, name: psp721_transfer <------ method from contracts crate
event: 22 <------ event from contracts crate
event: 721 <------ event from contracts crate
event: 42
event: 123
event: 321
```