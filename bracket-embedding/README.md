# Bracket-embedding

`bracket-lib` includes a system for embedding resources inside your binary (particularly useful for wasm builds).
This crate provides the supporting infrastructure for the embedding. It's not a lot of use on its own.

## Example of use

```rust
use bracket_embedding::prelude::*;

embedded_resource!(SOURCE_FILE, "embedding.rs");

fn main() {
   // This helper macro links the above embedding, allowing it to be accessed as a resource from various parts of the program.
   link_resource!(SOURCE_FILE, "embedding.rs");
}
```
