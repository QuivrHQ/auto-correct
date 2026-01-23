//! Benchmarks for the filter module

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use grammar_rs::prelude::*;
use grammar_rs::filter::{UrlFilter, CodeBlockFilter, QuotedTextFilter, DateFilter, NumberFilter};
use grammar_rs::core::filter::Filter;

const TEXT_WITH_URLS: &str = "Check out https://example.com/page and http://test.org/path. \
    Contact us at support@example.com or info@test.org for more information. \
    Visit www.example.net today!";

const TEXT_WITH_CODE: &str = "Use `cargo build` to compile. For more info see `rustc --help`. \
    Here's a code block:\n```rust\nfn main() {\n    println!(\"Hello\");\n}\n```\nThat's it.";

const TEXT_WITH_QUOTES: &str = r#"He said "hello world" to her. She replied "goodbye" quickly. \
    The French said « bonjour » and « au revoir ». \
    Smart quotes: "nice" and 'great'."#;

const TEXT_WITH_DATES: &str = "Meeting on 2024-01-15 at noon. Due by 01/15/2024. \
    Born on January 15th, 2024. European: 15.01.2024. \
    Events on March 1st and December 31st, 2025.";

const TEXT_WITH_NUMBERS: &str = "There are twenty-one items. She is thirty-five years old. \
    Only forty-two remain. We need fifty-eight more. \
    French: vingt-trois, trente-quatre, quarante-cinq.";

const MIXED_TEXT: &str = r#"
    Visit https://example.com for more info. Contact support@test.com.
    Use `cargo build --release` to compile the project.
    He said "this is important" during the meeting on 2024-03-15.
    There were twenty-one participants at the « conférence » in Paris.
    The deadline is January 15th, 2025. Email me at john@example.org.
"#;

fn bench_url_filter(c: &mut Criterion) {
    let filter = UrlFilter::new();

    c.bench_function("filters/url", |b| {
        b.iter(|| filter.find_masks(black_box(TEXT_WITH_URLS)))
    });
}

fn bench_code_filter(c: &mut Criterion) {
    let filter = CodeBlockFilter::new();

    c.bench_function("filters/code", |b| {
        b.iter(|| filter.find_masks(black_box(TEXT_WITH_CODE)))
    });
}

fn bench_quoted_filter(c: &mut Criterion) {
    let filter = QuotedTextFilter::new();

    c.bench_function("filters/quoted", |b| {
        b.iter(|| filter.find_masks(black_box(TEXT_WITH_QUOTES)))
    });
}

fn bench_date_filter(c: &mut Criterion) {
    let filter = DateFilter::new();

    c.bench_function("filters/date", |b| {
        b.iter(|| filter.find_masks(black_box(TEXT_WITH_DATES)))
    });
}

fn bench_number_filter(c: &mut Criterion) {
    let filter = NumberFilter::new();

    c.bench_function("filters/number", |b| {
        b.iter(|| filter.find_masks(black_box(TEXT_WITH_NUMBERS)))
    });
}

fn bench_filter_chain(c: &mut Criterion) {
    let filters = default_filters();

    c.bench_function("filters/chain_all", |b| {
        b.iter(|| filters.find_all_masks(black_box(MIXED_TEXT)))
    });
}

fn bench_filter_chain_sizes(c: &mut Criterion) {
    let filters = default_filters();

    let short = MIXED_TEXT;
    let medium = MIXED_TEXT.repeat(10);
    let long = MIXED_TEXT.repeat(100);

    let mut group = c.benchmark_group("filters/chain_size");

    group.bench_with_input(
        BenchmarkId::new("short", short.len()),
        short,
        |b, text| b.iter(|| filters.find_all_masks(black_box(text))),
    );

    group.bench_with_input(
        BenchmarkId::new("medium", medium.len()),
        &medium,
        |b, text| b.iter(|| filters.find_all_masks(black_box(text))),
    );

    group.bench_with_input(
        BenchmarkId::new("long", long.len()),
        &long,
        |b, text| b.iter(|| filters.find_all_masks(black_box(text))),
    );

    group.finish();
}

fn bench_pipeline_with_filters(c: &mut Criterion) {
    let pipeline_no_filters = Pipeline::new(
        SimpleTokenizer::new(),
        PassthroughAnalyzer::new(),
    )
    .with_checker(RuleChecker::new().with_english_rules());

    let pipeline_with_filters = Pipeline::new(
        SimpleTokenizer::new(),
        PassthroughAnalyzer::new(),
    )
    .with_checker(RuleChecker::new().with_english_rules())
    .with_default_filters();

    let mut group = c.benchmark_group("pipeline/filters");

    group.bench_function("without_filters", |b| {
        b.iter(|| pipeline_no_filters.check_text(black_box(MIXED_TEXT)))
    });

    group.bench_function("with_filters", |b| {
        b.iter(|| pipeline_with_filters.check_text(black_box(MIXED_TEXT)))
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_url_filter,
    bench_code_filter,
    bench_quoted_filter,
    bench_date_filter,
    bench_number_filter,
    bench_filter_chain,
    bench_filter_chain_sizes,
    bench_pipeline_with_filters,
);

criterion_main!(benches);
