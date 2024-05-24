use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::RngCore;
use zoc::Z;

criterion_group!(benches, benchmark);
criterion_main!(benches);

fn benchmark(c: &mut Criterion) {
    let mut vals = vec![0u8; 400];
    rand::thread_rng().fill_bytes(&mut vals);

    let mut it = vals.chunks(8).cycle().map(|slice| <[u8; 8]>::try_from(slice).unwrap());

    let mut group = c.benchmark_group("encode");
    group.bench_function("z", |b| b.iter(|| {
        let parts = it.next().unwrap();
        black_box(Z::interlace(&parts))
    }));
    group.finish();


    let mut vals = Vec::new();
    for _ in 0 .. 200 {
        vals.push(rand::random::<u64>())
    }

    let mut it = vals.iter().copied().cycle();

    let mut group = c.benchmark_group("decode");
    group.bench_function("z", |b| b.iter(|| {
        let z = it.next().unwrap();
        black_box(Z::<8, u8>::new(z).deinterlace())
    }));
    group.finish();
}
