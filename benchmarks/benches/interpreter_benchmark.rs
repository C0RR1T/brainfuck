extern crate interpreter;

use criterion::{criterion_group, criterion_main, Criterion};

use parser::hello_world;

fn interpreter(c: &mut Criterion) {
    c.bench_function("interpret hello world", |b| {
        b.iter(|| {
            interpreter::Interpreter::new().interpret_ins(&hello_world());
        })
    });
    c.bench_function("interpret file fizzbuzz", |b| {
        b.iter(|| {
            interpreter::Interpreter::new()
                .interpret_file_quiet("../brainfuck-example/fizzbuzz.bf");
        })
    });
    c.bench_function("interpret file hello-world", |b| {
        b.iter(|| {
            interpreter::Interpreter::new()
                .interpret_file_quiet("../brainfuck-example/hello-world.bf");
        })
    });
}

criterion_group!(benches, interpreter);
criterion_main!(benches);
