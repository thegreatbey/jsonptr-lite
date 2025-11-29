use jsonptr_lite::{ptr, ptr_mut};
use serde_json::json;
/*
#[test]
fn mutate_paths() {
    // change nested object value by pointer
    let mut v = json!({"a":{"b":0},"items":[1,2,3]});
    *ptr_mut(&mut v, "/a/b").unwrap() = json!(42);
    assert_eq!(ptr(&v, "/a/b").and_then(|x| x.as_i64()), Some(42));

    // change array element by pointer
    *ptr_mut(&mut v, "/items/1").unwrap() = json!(9);
    assert_eq!(ptr(&v, "/items/1").and_then(|x| x.as_i64()), Some(9));
}
*/

#[test]
fn basic_paths() {
    let v = json!({"a":{"b":3},"items":[10,20,30]});
    assert_eq!(ptr(&v, "/a/b").and_then(|x| x.as_i64()), Some(3));
    assert_eq!(ptr(&v, "/items/1").and_then(|x| x.as_i64()), Some(20));
}

#[test]
fn escapes_supported() {
    let v = json!({"a/b":7, "x~y": 9});
    assert_eq!(ptr(&v, "/a~1b").and_then(|x| x.as_i64()), Some(7));
    assert_eq!(ptr(&v, "/x~0y").and_then(|x| x.as_i64()), Some(9));
}

#[test]
fn invalid_cases() {
    let v = json!({"a":1});
    assert!(ptr(&v, "a").is_none()); // missing leading slash
    assert!(ptr(&v, "/~").is_none()); // bad escape
    assert!(ptr(&v, "/a/0").is_none()); // descend into non-container
}

#[test]
fn mutate_paths() {
    //use serde_json::json;

    let mut v = json!({"a":{"b":0},"items":[1,2,3]});

    // change nested object value by pointer
    *ptr_mut(&mut v, "/a/b").unwrap() = json!(42);
    assert_eq!(ptr(&v, "/a/b").and_then(|x| x.as_i64()), Some(42));

    // change array element by pointer
    *ptr_mut(&mut v, "/items/1").unwrap() = json!(9);
    assert_eq!(ptr(&v, "/items/1").and_then(|x| x.as_i64()), Some(9));
}
