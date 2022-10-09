#![allow(unused_variables)]

use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use num_format::{Buffer, Locale, ToFormattedString, WriteFormatted};

fn bench_usize(c: &mut Criterion) {
    let measurement_time = Duration::from_millis(5_000); // Default is 5 seconds
    let sample_size = 100; // Default is 100
    let warm_up_time = Duration::from_millis(3_000); // Default is 3 seconds
    let mut group = c.benchmark_group("usize");

    group.bench_function("std/to_string/10_000", |b| {
        b.iter(|| {
            let s = 10_000usize.to_string();
        })
    });
    group.bench_function("num-format/buffer/10_000", |b| {
        b.iter(|| {
            let mut buf = Buffer::default();
            buf.write_formatted(&10_000usize, &Locale::en);
        })
    });
    group.bench_function("num-format/write/10_000", |b| {
        let mut s = String::new();
        b.iter(|| {
            s.write_formatted(&10_000usize, &Locale::en).unwrap();
        })
    });
    group.bench_function("num-format/to_string/10_000", |b| {
        b.iter(|| {
            let s = 10_000usize.to_formatted_string(&Locale::en);
        })
    });
    group.bench_function("std/to_string/MAX", |b| {
        b.iter(|| {
            let s = std::usize::MAX.to_string();
        })
    });
    group.bench_function("num-format/buffer/MAX", |b| {
        b.iter(|| {
            let mut buf = Buffer::default();
            buf.write_formatted(&std::usize::MAX, &Locale::en);
        })
    });
    group.bench_function("num-format/write/MAX", |b| {
        let mut s = String::new();
        b.iter(|| {
            s.write_formatted(&std::usize::MAX, &Locale::en).unwrap();
        })
    });
    group.bench_function("num-format/to_string/MAX", |b| {
        b.iter(|| {
            let s = std::usize::MAX.to_formatted_string(&Locale::en);
        })
    });
    group.measurement_time(measurement_time);
    group.sample_size(sample_size);
    group.warm_up_time(warm_up_time);
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = bench_usize
}
criterion_main!(benches);
