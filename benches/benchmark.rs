extern crate rust_complex;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use rust_complex::complex::*;

fn bench_spiral_cartesian(c: &mut Criterion) -> std::io::Result<()> {
    c.bench_function("complex spiral as Cartesian", |b| {
        b.iter(|| {
            let mut c = Cartesian{r: 1.0, i: 0.0};

            for n in 0..1000 {
                c = (c * Polar{argument: 0.1, module: 1.01}).into();

            }
        })
    });

    Ok(())
}

fn bench_spiral_polar(c: &mut Criterion) -> std::io::Result<()> {
    c.bench_function("complex spiral as Polar", |b| {
        b.iter(|| {
            let mut c = Polar{argument: 0.0, module: 0.0};

            for n in 0..1000 {
                c = (c * Polar{argument: 0.1, module: 1.01});

                black_box(c);
            }
        })
    });

    Ok(())
}


fn bench_spirograph_cartesian(c: &mut Criterion) -> std::io::Result<()> {
    c.bench_function("complex spiral as Polar", |b| {
        b.iter(|| {
            let mut p = Cartesian{r: 3.0, i: 0.0};
            let mut center = Cartesian{r: 7.0, i: 0.0};

            for _ in 0..1000 {
                p = (p * Polar{argument: 0.1, module: 1.0}).into();
                center = (center * Polar{argument: 0.03, module: 1.0}).into();

                let r = p + center;

                black_box(r);
            }
        })
    });

    Ok(())
}

criterion_group!(
    benches,
    bench_spiral_cartesian,
    bench_spiral_polar,
    bench_spirograph_cartesian,
);

criterion_main!(benches);
