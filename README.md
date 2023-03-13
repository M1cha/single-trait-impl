# single-trait-impl

Provides a macro for declaring and implementing a trait at the same time. This
can be useful when you simply want to add functionality to an existing struct
but have no intention to implement it on other structs as well.

In other words, this macro reduces the verbosity of implementing your own trait
for a single struct:

```rust
#[single_trait_impl]
impl SocketAddrEx for std::net::SocketAddr {
    fn useful_function(&self) {
    }
}
```

This will expand to the following code:

```rust
pub trait SocketAddrEx {
    fn useful_function(&self);
}

impl SocketAddrEx for std::net::SocketAddr {
    fn useful_function(&self) {
    }
}
```
