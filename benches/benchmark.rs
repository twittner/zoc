use criterion::{black_box, criterion_group, criterion_main, Criterion};
use zoc::Z;

criterion_group!(benches, benchmark);
criterion_main!(benches);

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("interlace");

    let mut vals = Vec::new();
    for _ in 0 .. 200 {
        vals.push(rand::random::<u8>())
    }
    let mut it = vals.chunks(2).cycle().map(|slice| <[u8; 2]>::try_from(slice).unwrap());

    group.bench_function("[u8; 2]", |b| b.iter(|| {
        let parts = it.next().unwrap();
        black_box(Z::interlace(&parts))
    }));

    let mut it = vals.chunks(4).cycle().map(|slice| <[u8; 4]>::try_from(slice).unwrap());

    group.bench_function("[u8; 4]", |b| b.iter(|| {
        let parts = it.next().unwrap();
        black_box(Z::interlace(&parts))
    }));

    let mut it = vals.chunks(8).cycle().map(|slice| <[u8; 8]>::try_from(slice).unwrap());

    group.bench_function("[u8; 8]", |b| b.iter(|| {
        let parts = it.next().unwrap();
        black_box(Z::interlace(&parts))
    }));

    let mut vals = Vec::new();
    for _ in 0 .. 200 {
        vals.push(rand::random::<u32>())
    }
    let mut it = vals.chunks(2).cycle().map(|slice| <[u32; 2]>::try_from(slice).unwrap());

    group.bench_function("[u32; 2]", |b| b.iter(|| {
        let parts = it.next().unwrap();
        black_box(Z::interlace(&parts))
    }));

    let mut it = vals.chunks(4).cycle().map(|slice| <[u32; 4]>::try_from(slice).unwrap());

    group.bench_function("[u32; 4]", |b| b.iter(|| {
        let parts = it.next().unwrap();
        black_box(Z::interlace(&parts))
    }));

    let mut vals = Vec::new();
    for _ in 0 .. 200 {
        vals.push(rand::random::<u64>())
    }
    let mut it = vals.chunks(2).cycle().map(|slice| <[u64; 2]>::try_from(slice).unwrap());

    group.bench_function("[u64; 2]", |b| b.iter(|| {
        let parts = it.next().unwrap();
        black_box(Z::interlace(&parts))
    }));

    group.finish();


    let mut group = c.benchmark_group("deinterlace");

    let mut vals = Vec::new();
    for _ in 0 .. 200 {
        vals.push(rand::random::<u16>())
    }

    let mut it = vals.iter().copied().cycle();

    group.bench_function("[u8; 2]", |b| b.iter(|| {
        let z = it.next().unwrap();
        black_box(Z::<2, u8>::new(z).deinterlace())
    }));

    let mut vals = Vec::new();
    for _ in 0 .. 200 {
        vals.push(rand::random::<u32>())
    }

    let mut it = vals.iter().copied().cycle();

    group.bench_function("[u8; 4]", |b| b.iter(|| {
        let z = it.next().unwrap();
        black_box(Z::<4, u8>::new(z).deinterlace())
    }));

    let mut vals = Vec::new();
    for _ in 0 .. 200 {
        vals.push(rand::random::<u64>())
    }

    let mut it = vals.iter().copied().cycle();

    group.bench_function("[u8; 8]", |b| b.iter(|| {
        let z = it.next().unwrap();
        black_box(Z::<8, u8>::new(z).deinterlace())
    }));

    let mut it = vals.iter().copied().cycle();

    group.bench_function("[u32; 2]", |b| b.iter(|| {
        let z = it.next().unwrap();
        black_box(Z::<2, u32>::new(z).deinterlace())
    }));

    let mut vals = Vec::new();
    for _ in 0 .. 200 {
        vals.push(rand::random::<u128>())
    }

    let mut it = vals.iter().copied().cycle();

    group.bench_function("[u32; 4]", |b| b.iter(|| {
        let z = it.next().unwrap();
        black_box(Z::<4, u32>::new(z).deinterlace())
    }));

    let mut it = vals.iter().copied().cycle();

    group.bench_function("[u64; 2]", |b| b.iter(|| {
        let z = it.next().unwrap();
        black_box(Z::<2, u64>::new(z).deinterlace())
    }));

    group.finish();
}
