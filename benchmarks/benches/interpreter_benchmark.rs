extern crate interpreter;

use criterion::{criterion_group, criterion_main, Criterion};

use parser::hello_world;

fn interpreter(c: &mut Criterion) {
    c.bench_function("interpret hello world", |b| {
        b.iter(|| {
            interpreter::Interpreter::new().interpret(&hello_world());
        })
    });
    c.bench_function("interpret file fizzbuzz", |b| {
        b.iter(|| {
            interpreter::Interpreter::new()
                .interpret_file_quiet("../brainfuck-example/fizzbuzz.txt".to_string());
        })
    });
    c.bench_function("interpret file hello-world", |b| {
        b.iter(|| {
            interpreter::Interpreter::new()
                .interpret_file_quiet("../brainfuck-example/hello-world.txt".to_string());
        })
    });
}

criterion_group!(benches, interpreter);
criterion_main!(benches);
