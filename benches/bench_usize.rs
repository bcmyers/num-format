use std::time::Duration;

use criterion::{criterion_group, criterion_main, Benchmark, Criterion};
use itoa;
use num_format::{format::Locale, Buffer, ToFormattedString, WriteFormatted};

fn bench_usize(c: &mut Criterion) {
    let measurement_time = Duration::from_millis(5_000); // Default is 5 seconds
    let sample_size = 100; // Default is 100
    let warm_up_time = Duration::from_millis(3_000); // Default is 3 seconds

    #[rustfmt::skip]
    c.bench(
        "usize",
        Benchmark::new("nothing", |b| b.iter(|| ()))

            // 10_000
            .with_function("std/10_000", |b| b.iter(|| {
                10_000usize.to_string()
            }))
            .with_function("num-format/buffer/10_000", |b| b.iter(|| {
                let mut buf = Buffer::default();
                buf.write_formatted(&10_000usize, &Locale::en);
                buf.as_str().to_string();
            }))
            .with_function("num-format/write_formatted/10_000", |b| b.iter(|| {
                let mut s = String::with_capacity(256);
                s.write_formatted(&10_000usize, &Locale::en).unwrap();
                s
            }))
            .with_function("num-format/to_formatted_string/10_000", |b| b.iter(|| {
                10_000usize.to_formatted_string(&Locale::en)
            }))
            .with_function("itoa/10_000", |b| b.iter(|| {
                let mut buf = itoa::Buffer::new();
                let s = buf.format(10_000usize);
                s.to_string()
            }))

            // MAX
            .with_function("std/MAX", |b| b.iter(|| {
                std::usize::MAX.to_string()
            }))
            .with_function("num-format/buffer/MAX", |b| b.iter(|| {
                let mut buf = Buffer::default();
                buf.write_formatted(&std::usize::MAX, &Locale::en);
                buf.as_str().to_string();
            }))
            .with_function("num-format/write_formatted/MAX", |b| b.iter(|| {
                let mut s = String::with_capacity(256);
                s.write_formatted(&std::usize::MAX, &Locale::en).unwrap();
                s
            }))
            .with_function("num-format/to_formatted_string/MAX", |b| b.iter(|| {
                std::usize::MAX.to_formatted_string(&Locale::en)
            }))
            .with_function("itoa/MAX", |b| b.iter(|| {
                let mut buf = itoa::Buffer::new();
                let s = buf.format(std::usize::MAX);
                s.to_string()
            }))

            .measurement_time(measurement_time)
            .sample_size(sample_size)
            .warm_up_time(warm_up_time),
    );
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = bench_usize
}
criterion_main!(benches);
