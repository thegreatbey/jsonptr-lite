use serde_json::{json, Value};
use jsonptr_lite::ptr;

fn main() {
    // show basic object path
    let v = json!({ "a": { "b": 3 }});
    println!("{}", ptr(&v, "/a/b").and_then(Value::as_i64).unwrap()); // 3

    // show array indexing
    let v = json!({ "items": [10, 20, 30] });
    println!("{}", ptr(&v, "/items/2").and_then(Value::as_i64).unwrap()); // 30

    // show escapes for "/" and "~"
    let v = json!({ "a/b": 7, "x~y": 9 });
    println!("{}", ptr(&v, "/a~1b").and_then(Value::as_i64).unwrap()); // 7
    println!("{}", ptr(&v, "/x~0y").and_then(Value::as_i64).unwrap()); // 9
}


