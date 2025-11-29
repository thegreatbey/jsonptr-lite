# jsonptr-lite

Tiny JSON Pointer lookups for `serde_json::Value` (RFC 6901), dependency-light and fast to use.

- one function: `ptr(&Value, &str) -> Option<&Value>`
- supports escapes in tokens: `~1` → `/`, `~0` → `~`
- returns `None` for invalid pointers or missing paths

## Examples

```rust
use serde_json::{json, Value};
use jsonptr_lite::ptr;

let v = json!({ "a": { "b": 3 }});
assert_eq!(ptr(&v, "/a/b").and_then(Value::as_i64), Some(3));
```

ptr_mut example
```rust
use serde_json::{json, Value};
use jsonptr_lite::{ptr, ptr_mut};

//change nested object value by pointer
let mut v = json!({"a":{"b":0},"items":[1,2,3]});
*ptr_mut(&mut v, "/a/b").unwrap() = json!(42);
assert_eq!(ptr(&v, "/a/b").and_then(Value::as_i64), Some(42));

//change array element by pointer
*ptr_mut(&mut v, "/items/1").unwrap() = json!(9);
assert_eq!(ptr(&v, "/items/1").and_then(Value::as_i64), Some(9));
```


Arrays and escapes:

```rust
use serde_json::{json, Value};
use jsonptr_lite::ptr;

let v = json!({
  "items": [10, 20, 30],
  "a/b": 7,
  "x~y": 9
});

assert_eq!(ptr(&v, "/items/1").and_then(Value::as_i64), Some(20));
assert_eq!(ptr(&v, "/a~1b").and_then(Value::as_i64), Some(7)); // "/" in key
assert_eq!(ptr(&v, "/x~0y").and_then(Value::as_i64), Some(9)); // "~" in key
```

## License

Licensed under either of

- Apache License, Version 2.0
- MIT license

at your option.


