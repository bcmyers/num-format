use criterion::{criterion_group, criterion_main, Criterion};
use num_display::test_func;
use num_display::DisplayInteger;
use separator::Separatable;

fn criterion_benchmark(c: &mut Criterion) {
    // c.bench_function("1000, to_string", move |b| {
    //     b.iter(|| {
    //         let x: usize = 1000;
    //         x.to_string();
    //     })
    // });
    // c.bench_function("1000, to_formatted", move |b| {
    //     b.iter(|| {
    //         let x: usize = 1000;
    //         x.to_formatted();
    //     })
    // });
    // c.bench_function("1000, separated_string", move |b| {
    //     b.iter(|| {
    //         let x: usize = 1000;
    //         x.separated_string();
    //     })
    // });

    // c.bench_function("u64::MAX, to_string", move |b| {
    //     b.iter(|| std::u64::MAX.to_string())
    // });
    // c.bench_function("u64::MAX, to_formatted", move |b| {
    //     b.iter(|| std::u64::MAX.to_formatted())
    // });
    // c.bench_function("u64::MAX, separated_string", move |b| {
    //     b.iter(|| std::u64::MAX.separated_string())
    // });

    c.bench_function("u128::MAX, to_string", move |b| {
        b.iter(|| std::u32::MAX.to_string())
    });
    c.bench_function("u128::MAX, test_func", move |b| {
        b.iter(|| test_func(std::u32::MAX))
    });
    c.bench_function("u128::MAX, to_formatted", move |b| {
        b.iter(|| std::u32::MAX.to_formatted())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
