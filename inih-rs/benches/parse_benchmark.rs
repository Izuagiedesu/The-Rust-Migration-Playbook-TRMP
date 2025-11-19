use criterion::{black_box, criterion_group, criterion_main, Criterion};
use inih_rs::Ini;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn benchmark_ini_parsing(c: &mut Criterion) {
    // 1. Setup: Create a test INI file
    let test_content = "
[section1]
key1=value1
key2=value2
[section2]
key3=value3
key4=value4
";
    let path = Path::new("bench_test.ini");
    let mut file = File::create(path).unwrap();
    file.write_all(test_content.as_bytes()).unwrap();

    // 2. Benchmark: Measure the `from_file` function
    //    We use `black_box` to stop the compiler from cheating (optimizing too much)
    c.bench_function("parse_ini_file", |b| {
        b.iter(|| {
            Ini::from_file(black_box(path)).unwrap();
        })
    });

    // 3. Cleanup: Delete the file
    std::fs::remove_file(path).unwrap();
}

criterion_group!(benches, benchmark_ini_parsing);
criterion_main!(benches);