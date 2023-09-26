use criterion::*;
use moo_math::Moo;

pub fn bench(cr: &mut Criterion) {
    let moo = Moo::new(|_| {});
    cr.bench_function("hello", |b| {
        b.iter(|| {
            let _ = moo.parse("1 + 4.9 ^ 0.2 * x");
        });
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
