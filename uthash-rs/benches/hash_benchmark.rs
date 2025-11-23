use criterion::{black_box, criterion_group, criterion_main, Criterion};
use uthash_rs::UserDatabase;

fn benchmark_operations(c: &mut Criterion) {
    // Benchmark 1: Adding Users
    c.bench_function("add_1000_users", |b| {
        b.iter(|| {
            let mut db = UserDatabase::new();
            for i in 0..1000 {
                // black_box prevents compiler optimization cheating
                db.add(black_box(i), black_box("User")).unwrap();
            }
        })
    });

    // Benchmark 2: Finding Users
    // We create a populated DB first so we just measure lookup speed
    let mut setup_db = UserDatabase::new();
    for i in 0..1000 {
        setup_db.add(i, "User").unwrap();
    }

    c.bench_function("find_1000_users", |b| {
        b.iter(|| {
            for i in 0..1000 {
                black_box(setup_db.find(black_box(i)));
            }
        })
    });
}

criterion_group!(benches, benchmark_operations);
criterion_main!(benches);