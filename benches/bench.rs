#[macro_use]
extern crate criterion;

use criterion::{BenchmarkId, Criterion, Throughput};
use snakecase::ascii::to_snakecase as to_snakecase_ascii;
use snakecase::unicode::to_snakecase as to_snakecase_unicode;

fn bench_ascii(c: &mut Criterion) {
    let mut group = c.benchmark_group("ascii");

    for (name, input) in [
        ("ascii_owned_simple", "sample text"),
        ("ascii_borrowed_simple", "sample_text"),
        ("ascii_owned_long", "inviteYourCustomersAddInvites"),
        ("ascii_borrowed_long", "invite_your_customers_add_invites"),
        (
            "ascii_owned_long_special_chars",
            "FOO:BAR$BAZ__Sample    Text___",
        ),
        ("ascii_owned_unicode", "ẞ•¶§ƒ˚foo˙∆˚¬"),
        ("ascii_borrowed_unicode", "ß_ƒ_foo"),
        ("ascii_digit_uppercase", "5TEst"),
        ("ascii_starts_with_garbage", "@%#&5TEst"),
        ("ascii_complex_random", "lk0B@bFmjrLQ_Z6YL"),
    ]
    .iter()
    {
        group.throughput(Throughput::Bytes(input.len() as u64));
        group.bench_with_input(BenchmarkId::new(*name, input), input, |b, input| {
            b.iter(|| to_snakecase_ascii(*input))
        });
    }

    group.finish();
}

fn bench_unicode(c: &mut Criterion) {
    let mut group = c.benchmark_group("unicode");

    for (name, input) in [
        ("unicode_owned_simple", "sample text"),
        ("unicode_borrowed_simple", "sample_text"),
        ("unicode_borrowed_long", "invite_your_customers_add_invites"),
        (
            "unicode_owned_long_special_chars",
            "FOO:BAR$BAZ__Sample    Text___",
        ),
        ("unicode_owned_unicode", "ẞ•¶§ƒ˚foo˙∆˚¬"),
        ("unicode_borrowed_unicode", "ß_ƒ_foo"),
        ("unicode_digit_uppercase", "5TEst"),
        ("unicode_starts_with_garbage", "@%#&5TEst"),
        ("unicode_complex_random", "lk0B@bFmjrLQ_Z6YL"),
    ]
    .iter()
    {
        group.throughput(Throughput::Bytes(input.len() as u64));
        group.bench_with_input(BenchmarkId::new(*name, input), input, |b, input| {
            b.iter(|| to_snakecase_unicode(*input))
        });
    }

    group.finish();
}

criterion_group!(benches, bench_ascii, bench_unicode);
criterion_main!(benches);
