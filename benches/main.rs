#[macro_use]
extern crate bencher;
#[macro_use]
extern crate lazy_static;

use bencher::Bencher;
use bhh_rs::*;

const SIZE: usize = 1_000_000;
const POINTS: usize = 100;
const N_TESTS: usize = 100;
const OBJECT_POSITION_RADIUS: f32 = 50.;
const MESH_RADIUS: f32 = 1.;

lazy_static! {
    static ref SHARED_MESH: Mesh = Mesh::new(POINTS, MESH_RADIUS);
    static ref ITEMS: Vec<Object<'static>> = {
        use rand::Rng;

        let mut rng = rand::thread_rng();
        let mut coord = move || rng.gen_range(-OBJECT_POSITION_RADIUS, OBJECT_POSITION_RADIUS);

        (0..SIZE)
            .map(|_| Object {
                mesh: &SHARED_MESH,
                position: Float3 {
                    x: coord(),
                    y: coord(),
                    z: coord(),
                },
            })
            .collect()
    };
    static ref UNORDERED: Vec<AABB> =
        { ITEMS.iter().map(|x| { x.get_calculated_aabb() }).collect() };
    static ref ORDERED: Vec<AABB> = {
        let mut objects = UNORDERED.clone();
        bhh_sort(objects.as_mut_slice());
        objects
    };
}

fn bench_sorting(bencher: &mut Bencher) {
    bencher.iter(|| {
        let mut items = UNORDERED.clone();
        bhh_sort(&mut items);
    });
}

fn naive_search(items: &[AABB], query: &AABB) -> u32 {
    items.iter().filter(|a| a.intersects(query)).count() as u32
}

fn bench_naive_ordered(bencher: &mut Bencher) {
    bencher.iter(|| {
        let mut n: u32 = 0;

        for q in UNORDERED.iter().take(N_TESTS) {
            n += naive_search(ORDERED.as_slice(), q);
        }

        assert_eq!(n, bencher::black_box(n));
    });
}

fn bench_naive_unordered(bencher: &mut Bencher) {
    bencher.iter(|| {
        let mut n: u32 = 0;

        for q in UNORDERED.iter().take(N_TESTS) {
            n += naive_search(UNORDERED.as_slice(), q);
        }

        assert_eq!(n, bencher::black_box(n));
    });
}

fn bench_bhh_unordered(bencher: &mut Bencher) {
    bencher.iter(|| {
        let mut objects = UNORDERED.clone();
        bhh_sort(objects.as_mut_slice());

        let mut n: u32 = 0;

        for q in UNORDERED.iter().take(N_TESTS) {
            n += bhh_search(objects.as_slice(), q);
        }

        assert_eq!(n, bencher::black_box(n));
    });
}

fn bench_bhh_ordered(bencher: &mut Bencher) {
    bencher.bench_n(3, |_| {});

    bencher.iter(|| {
        let mut n: u32 = 0;

        for q in UNORDERED.iter().take(N_TESTS) {
            n += bhh_search(ORDERED.as_slice(), q);
        }

        assert_eq!(n, bencher::black_box(n));
    });
}

benchmark_group!(
    benches,
    bench_bhh_ordered,
    bench_naive_ordered,
    bench_bhh_unordered,
    bench_naive_unordered,
    bench_sorting,
);
benchmark_main!(benches);

