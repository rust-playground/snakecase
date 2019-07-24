#[macro_use]
extern crate criterion;

use criterion::{Benchmark, Criterion, Throughput};
use snakecase::ascii::to_snakecase as to_snakecase_ascii;
use snakecase::unicode::to_snakecase as to_snakecase_unicode;

fn criterion_benchmark(c: &mut Criterion) {
    macro_rules! snakecase_ascii_bench {
        ($name:expr,$s:expr) => {
            c.bench(
                "ascii",
                Benchmark::new($name, |b| b.iter(|| to_snakecase_ascii($s)))
                    .throughput(Throughput::Bytes($s.as_bytes().len() as u32)),
            );
        };
    }
    snakecase_ascii_bench!("ascii_owned_simple", "sample text");
    snakecase_ascii_bench!("ascii_borrowed_simple", "sample_text");
    snakecase_ascii_bench!("ascii_owned_long", "inviteYourCustomersAddInvites");
    snakecase_ascii_bench!("ascii_borrowed_long", "invite_your_customers_add_invites");
    snakecase_ascii_bench!(
        "ascii_owned_long_special_chars",
        "FOO:BAR$BAZ__Sample    Text___"
    );
    snakecase_ascii_bench!("ascii_owned_unicode", "ẞ•¶§ƒ˚foo˙∆˚¬");
    snakecase_ascii_bench!("ascii_borrowed_unicode", "ß_ƒ_foo");
    snakecase_ascii_bench!("ascii_digit_uppercase", "5TEst");
    snakecase_ascii_bench!("ascii_starts_with_garbage", "@%#&5TEst");
    snakecase_ascii_bench!("ascii_complex_random", "lk0B@bFmjrLQ_Z6YL");

    macro_rules! snakecase_bench {
        ($name:expr,$s:expr) => {
            c.bench(
                "unicode",
                Benchmark::new($name, |b| b.iter(|| to_snakecase_unicode($s)))
                    .throughput(Throughput::Bytes($s.as_bytes().len() as u32)),
            );
        };
    }
    snakecase_bench!("unicode_owned_simple", "sample text");
    snakecase_bench!("unicode_borrowed_simple", "sample_text");
    snakecase_bench!("unicode_borrowed_long", "invite_your_customers_add_invites");
    snakecase_bench!(
        "unicode_owned_long_special_chars",
        "FOO:BAR$BAZ__Sample    Text___"
    );
    snakecase_bench!("unicode_owned_unicode", "ẞ•¶§ƒ˚foo˙∆˚¬");
    snakecase_bench!("unicode_borrowed_unicode", "ß_ƒ_foo");
    snakecase_bench!("unicode_digit_uppercase", "5TEst");
    snakecase_bench!("unicode_starts_with_garbage", "@%#&5TEst");
    snakecase_bench!("unicode_complex_random", "lk0B@bFmjrLQ_Z6YL");
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
