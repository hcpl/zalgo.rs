```rust,skt-extern-crate-only
{}

fn main() {{ }}
```

```rust,skt-cfg-std-all-code-in-main
extern crate zalgo;

#[allow(unused_variables)]
fn main() {{
    #[cfg(feature = "std")]
    {{
        {}
    }}
}}
```
