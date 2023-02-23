use criterion::{criterion_group, criterion_main, Criterion};
use std::{thread, time};

fn quick_bug(c: &mut Criterion) {
    let one_second = time::Duration::from_secs(1);
    let six_seconds = time::Duration::from_secs(6);
    let mut group = c.benchmark_group("> 5 second quick bug test");
    group.sample_size(10);
    group.bench_function("1 seconds", |b| b.iter(|| thread::sleep(one_second)));
    group.bench_function("6 seconds", |b| b.iter(|| thread::sleep(six_seconds)));
    group.finish();
}

criterion_group!(benches, quick_bug);
criterion_main!(benches);
