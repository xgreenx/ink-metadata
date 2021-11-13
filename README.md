This repository shows how ink! can implement aggregation of metadata from crates. 

On startup of the metadata binary(created by `cargo-contract`) `on_startup` 
macro will init the static variable of metadata defined in 
ink!(it also works from `rlib` crates). 

After ink! can process this variable to generate an ABI. 
This schema allows defining events independently
(And you also can use the same idea to generate ABI for all methods defined by the user).

You should build it with enabled `lto`(it requires also enabling of `embed-bitcode`).
TO test this example you need to run:
```shell
cd hello_cargo
RUSTFLAGS="-Clto -Cembed-bitcode=yes" cargo build && ./target/debug/hello_cargo
```

You should see the next output:
```shell
Version: 3
selector: 9, name: value
selector: 1111111, name: some_ink_method
selector: 131313, name: psp22_balance_of
selector: 8, name: flip
selector: 141414, name: psp22_transfer
selector: 0, name: fallback_method
event: 111
event: 42
event: 123
event: 321
```