use criterion::{black_box, criterion_group, criterion_main, Criterion};
use zoc::{search::range, Bbox, Z};

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


    let mut group = c.benchmark_group("bbox");

    let bbox = Bbox::<2, u32>::new(Z::new(2790), Z::new(1023435));

    group.bench_function("litmax", |b| b.iter(|| {
        black_box(bbox.litmax(&Z::new(58734)));
    }));

    group.bench_function("bigmin", |b| b.iter(|| {
        black_box(bbox.bigmin(&Z::new(58734)));
    }));

    group.finish();

    let mut group = c.benchmark_group("search");

    let mut vec = Vec::new();
    for x in 0 .. 32u32 {
        for y in 0 .. 32u32 {
            vec.push(Z::from([y, x]));
        }
    }
    vec.sort_unstable();

    group.bench_function("([5, 7], [17, 21])", |b| b.iter(|| {
        black_box(range(vec.as_slice(), [5, 7], [17, 21]).count());
    }));

    group.bench_function("([5, 7], [17012, 213413])", |b| b.iter(|| {
        black_box(range(vec.as_slice(), [5, 7], [17012, 213413]).count());
    }));

    group.finish();
}
