//! # Mutate JSON with JSON Pointer example shows how to mutate a JSON value using a JSON Pointer.
use serde_json::{json, Value};
use jsonptr_lite::{ptr, ptr_mut};

fn main() {
    //change nested object value by pointer
    let mut v = json!({"a":{"b":0},"items":[1,2,3]});
    *ptr_mut(&mut v, "/a/b").unwrap() = json!(42);
    println!("{}", ptr(&v, "/a/b").and_then(Value::as_i64).unwrap()); // 42

    //change array element by pointer
    *ptr_mut(&mut v, "/items/1").unwrap() = json!(9);
    println!("{}", ptr(&v, "/items/1").and_then(Value::as_i64).unwrap()); // 9
}