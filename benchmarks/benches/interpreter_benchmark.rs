extern crate interpreter;

use criterion::{criterion_group, criterion_main, Criterion};

const FIZZBUZZ: &str = include_str!("../../brainfuck-example/fizzbuzz.bf");
const HELLO_WORD: &str = include_str!("../../brainfuck-example/hello-world.bf");
const TOWER_OF_HANOI: &str = include_str!("../../brainfuck-example/towers-of-hanoi.bf");

fn interpreter(c: &mut Criterion) {
    c.bench_function("interpret hello world no-opt", |b| {
        b.iter(|| {
            interpreter::Interpreter::new().interpret(HELLO_WORD, false);
        })
    });
    c.bench_function("interpret file FIZZBUZZ", |b| {
        b.iter(|| {
            interpreter::Interpreter::new().interpret(FIZZBUZZ, true);
        })
    });
    c.bench_function("interpret file hello-world", |b| {
        b.iter(|| {
            interpreter::Interpreter::new().interpret(HELLO_WORD, true);
        })
    });

    c.bench_function("interpret tower-of-hanoi", |b| {
        b.iter(|| interpreter::Interpreter::new().interpret(TOWER_OF_HANOI, true))
    });
    c.bench_function("interpret tower-of-hanoi no-opt", |b| {
        b.iter(|| interpreter::Interpreter::new().interpret(TOWER_OF_HANOI, false))
    });
}

criterion_group!(benches, interpreter);
criterion_main!(benches);
