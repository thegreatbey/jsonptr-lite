//! Tiny JSON Pointer lookups for `serde_json::Value`.
//!
//! Supports RFC 6901 escape rules for path tokens:
//! - `~1` becomes `/`
//! - `~0` becomes `~`
//!
//! This is intentionally minimal: one function, no allocations beyond token unescaping.
//! Returns `None` for invalid pointers or missing paths.
//!
//! # Examples
//! ```
//! use serde_json::{json, Value};
//! use jsonptr_lite::ptr;
//!
//! let v = json!({ "a": { "b": 3 }});
//! assert_eq!(ptr(&v, "/a/b").and_then(Value::as_i64), Some(3));
//!
//! // array index
//! let v = json!({ "items": [10, 20, 30] });
//! assert_eq!(ptr(&v, "/items/1").and_then(Value::as_i64), Some(20));
//!
//! // escaped slash in key name: "/"
//! let v = json!({ "a/b": 7 });
//! assert_eq!(ptr(&v, "/a~1b").and_then(Value::as_i64), Some(7));
//!
//! // empty pointer returns the whole value
//! let v = json!(42);
//! assert_eq!(ptr(&v, "").and_then(Value::as_i64), Some(42));
//! ```

//! change value via pointer
//! use serde_json::{json, Value};
//! use jsonptr_lite::{ptr, ptr_mut};
//! let mut v = json!({"a":{"b":0}});
//! *ptr_mut(&mut v, "/a/b").unwrap() = json!(42);
//! assert_eq!(ptr(&v, "/a/b").and_then(Value::as_i64), Some(42));
use serde_json::Value;





/// Lookup a JSON value by JSON Pointer (RFC 6901).
///
/// - empty string `""` returns the input `value`
/// - each path segment is separated by `/`
/// - escape rules in segments: `~1` → `/`, `~0` → `~`
/// - returns `None` if the path cannot be followed
pub fn ptr<'a>(value: &'a Value, pointer: &str) -> Option<&'a Value> {
    // empty pointer means the whole document
    if pointer.is_empty() {
        return Some(value);
    }
    // pointer must begin with "/" per RFC 6901
    if !pointer.starts_with('/') {
        return None;
    }

    let mut current = value;
    // skip the leading empty segment before the first "/"
    for raw_token in pointer.split('/').skip(1) {
        // decode "~1" -> "/" and "~0" -> "~"
        let token = unescape_token(raw_token)?;

        match current {
            Value::Object(map) => {
                // step into object by key
                current = map.get(&token)?;
            }
            Value::Array(items) => {
                // step into array by index
                let idx: usize = token.parse().ok()?;
                current = items.get(idx)?;
            }
            _ => {
                // cannot descend into primitives
                return None;
            }
        }
    }
    Some(current)
}

// turn token escapes into their characters
// "~1" becomes "/" and "~0" becomes "~"
// any other "~" sequence is invalid
fn unescape_token(token: &str) -> Option<String> {
    // fast path: no tilde, nothing to unescape
    if !token.contains('~') {
        return Some(token.to_owned());
    }

    let mut out = String::with_capacity(token.len());
    let mut chars = token.chars();
    while let Some(c) = chars.next() {
        if c == '~' {
            match chars.next() {
                Some('0') => out.push('~'),
                Some('1') => out.push('/'),
                _ => return None,
            }
        } else {
            out.push(c);
        }
    }
    Some(out)
}

pub fn ptr_mut<'a>(value: &'a mut Value, pointer: &str) -> Option<&'a mut Value> {
    // empty pointer means the whole document
    if pointer.is_empty() {
        return Some(value);
    }
    // pointer must begin with "/" per RFC 6901
    if !pointer.starts_with('/') {
        return None;
    }

    let mut current = value;
    // skip the leading empty segment before the first "/"
    for raw_token in pointer.split('/').skip(1) {
        // decode "~1" -> "/" and "~0" -> "~"
        let token = unescape_token(raw_token)?;

        match current {
            Value::Object(map) => {
                // step into object by key
                current = map.get_mut(&token)?;
            }
            Value::Array(items) => {
                // step into array by index
                let idx: usize = token.parse().ok()?;
                current = items.get_mut(idx)?;
            }
            _ => {
                // cannot descend into primitives
                return None;
            }
        }
    }
    Some(current)
}

#[cfg(test)]
mod tests {
    use super::ptr;
    use serde_json::json;

    #[test]
    fn root_pointer() {
        let v = json!(123);
        assert_eq!(ptr(&v, "").and_then(|x| x.as_i64()), Some(123));
    }

    #[test]
    fn object_path() {
        let v = json!({"a":{"b":3}});
        assert_eq!(ptr(&v, "/a/b").and_then(|x| x.as_i64()), Some(3));
        assert!(ptr(&v, "/a/x").is_none());
    }

    #[test]
    fn array_index() {
        let v = json!({"items":[10,20,30]});
        assert_eq!(ptr(&v, "/items/0").and_then(|x| x.as_i64()), Some(10));
        assert_eq!(ptr(&v, "/items/2").and_then(|x| x.as_i64()), Some(30));
        assert!(ptr(&v, "/items/3").is_none());
        assert!(ptr(&v, "/items/-1").is_none());
    }

    #[test]
    fn escapes() {
        let v = json!({"a/b": 7, "x~y": 9});
        assert_eq!(ptr(&v, "/a~1b").and_then(|x| x.as_i64()), Some(7));
        assert_eq!(ptr(&v, "/x~0y").and_then(|x| x.as_i64()), Some(9));
    }

    #[test]
    fn invalid_pointer() {
        let v = json!({"a": 1});
        assert!(ptr(&v, "a").is_none()); // missing leading slash
        assert!(ptr(&v, "/~").is_none()); // bad escape
        assert!(ptr(&v, "/a/0").is_none()); // descending into non-container
    }
}


