use criterion::{black_box, criterion_group, criterion_main, Criterion};
use diplomat_example::{data_provider, decimal, fixed_decimal, locale};

pub fn criterion_benchmark_locale(c: &mut Criterion) {
    c.bench_function("create locale", |b| {
        b.iter(|| black_box(locale::ffi::ICU4XLocale::new("en".as_bytes())))
    });
}

pub fn criterion_benchmark_provider(c: &mut Criterion) {
    c.bench_function("create provider", |b| {
        b.iter(|| black_box(data_provider::ffi::ICU4XDataProvider::new_static()))
    });
}

pub fn criterion_benchmark_options(c: &mut Criterion) {
    c.bench_function("create options", |b| {
        b.iter(|| black_box(decimal::ffi::ICU4XFixedDecimalFormatterOptions::default()))
    });
}

pub fn criterion_benchmark_decimal(c: &mut Criterion) {
    c.bench_function("create decimal", |b| {
        b.iter(|| fixed_decimal::ffi::ICU4XFixedDecimal::new(black_box(123)))
    });
}

pub fn criterion_benchmark_formatter(c: &mut Criterion) {
    let loc = locale::ffi::ICU4XLocale::new("en".as_bytes());
    let prov = data_provider::ffi::ICU4XDataProvider::new_static();
    c.bench_function("create formatter", |b| {
        b.iter(|| {
            let options = decimal::ffi::ICU4XFixedDecimalFormatterOptions::default();
            black_box(decimal::ffi::ICU4XFixedDecimalFormatter::try_new(
                &loc, &prov, options,
            ))
        })
    });
}

pub fn criterion_benchmark_format(c: &mut Criterion) {
    let x = fixed_decimal::ffi::ICU4XFixedDecimal::new(black_box(123));
    let loc = locale::ffi::ICU4XLocale::new("en".as_bytes());
    let options = decimal::ffi::ICU4XFixedDecimalFormatterOptions::default();
    let prov = data_provider::ffi::ICU4XDataProvider::new_static();
    let formatter = decimal::ffi::ICU4XFixedDecimalFormatter::try_new(&loc, &prov, options)
        .expect("Failed to create formatter");
    c.bench_function("format", |b| {
        b.iter(|| {
            unsafe {
                let mut ptr = diplomat_runtime::diplomat_buffer_write_create(10);
                let writeable = ptr
                    .as_mut()
                    .expect("Tried to get null pointer as diplomat writeable");

                black_box(formatter.format_write(&x, writeable));
                diplomat_runtime::diplomat_buffer_write_destroy(ptr);
            };
        })
    });
}

criterion_group!(
    benches,
    criterion_benchmark_locale,
    criterion_benchmark_provider,
    criterion_benchmark_options,
    criterion_benchmark_decimal,
    criterion_benchmark_formatter,
    criterion_benchmark_format
);
criterion_main!(benches);
