use criterion::{black_box, criterion_group, criterion_main, Criterion};
use jsonptr_lite::ptr;
use serde_json::json;

//measure lookup cost for a nested path with an array index
fn bench_ptr(c: &mut Criterion) {
    let v = json!({ "a": { "b": { "c": [0,1,2,3,4,5,6,7,8,9] } } });

    c.bench_function("ptr /a/b/c/9", |b| {
        b.iter(|| {
            let _ = black_box(ptr(&v, "/a/b/c/9"));
        })
    });
}

criterion_group!(benches, bench_ptr);
criterion_main!(benches);
