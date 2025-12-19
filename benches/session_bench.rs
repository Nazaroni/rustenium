use criterion::{criterion_group, criterion_main, Criterion};

fn session_creation_benchmark(c: &mut Criterion) {
    // Note: Criterion does not support async directly on stable.
    // Instead, block_on the async code inside the closure.
    let rt = tokio::runtime::Runtime::new().unwrap();
    c.bench_function("create_webdriver_session", |b| {
        b.iter(|| {
            rt.block_on(async {
                // TODO: add actual benchmarking code here
            })
        })
    });
}

criterion_group!(benches, session_creation_benchmark);
criterion_main!(benches);
