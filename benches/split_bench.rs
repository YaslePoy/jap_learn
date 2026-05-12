use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_split(c: &mut Criterion) {
    let input = "data_1";

    c.bench_function("split_with_comma", |b| {
        b.iter(|| {
            // black_box запрещает компилятору оптимизировать результат
            black_box(split_with_coma(black_box(input)))
        })
    });
}

fn bench_split_new(c: &mut Criterion) {
    let input = "data_1";

    c.bench_function("split_with_comma gpt", |b| {
        b.iter(|| {
            // black_box запрещает компилятору оптимизировать результат
            black_box(split_with_coma(black_box(input)))
        })
    });
}


pub fn split_with_coma(ts: &str) -> Vec<String> {
    ts.split(',')
        .map(|ts| ts.to_lowercase().trim().to_string())
        .filter(|t| !t.is_empty())
        .collect::<Vec<String>>()
}

pub fn new_split_with_coma(ts: &str) -> Vec<String> {
    ts.split(',')
        .map(|s| s.trim())               // 0 аллокаций, просто срез
        .filter(|s| !s.is_empty())       // отбрасываем пустые до аллокации
        .map(|s| s.to_lowercase())       // 1 аллокация на валидный токен
        .collect()
}

// Можно добавить несколько бенчмарков в группу
criterion_group!(benches, bench_split, bench_split_new);
criterion_main!(benches);