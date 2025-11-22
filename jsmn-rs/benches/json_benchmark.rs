use criterion::{black_box, criterion_group, criterion_main, Criterion};
use jsmn_rs::{JsmnParser, Token};

fn benchmark_json_parsing(c: &mut Criterion) {
    // 1. Setup: A simple JSON string
    let json = black_box("{\"name\": \"Keshee\", \"type\": \"Consultancy\", \"id\": 123, \"active\": true}");
    
    // 2. Benchmark: Measure the `parse` function
    c.bench_function("parse_json_small", |b| {
        b.iter(|| {
            let mut parser = JsmnParser::new();
            // Allocate on stack (fastest possible memory)
            let mut tokens = [Token::default(); 10]; 
            parser.parse(json, &mut tokens).unwrap();
        })
    });
}

criterion_group!(benches, benchmark_json_parsing);
criterion_main!(benches);