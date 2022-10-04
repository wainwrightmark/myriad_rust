use criterion::{criterion_group, criterion_main, Criterion};
use myriad::core::prelude::*;

criterion_group!(benches, bench_find_solutions);
criterion_main!(benches);

fn bench_find_solutions(c: &mut Criterion) {
    let mut group = c.benchmark_group("solver");
    group.sample_size(10);
    group.bench_function("Count puns", |bench| {
        bench.iter(|| create_boards_and_solve(10))
    });
    group.finish()
}

fn create_boards_and_solve(number_of_boards: usize) {
    let solve_settings = SolveSettings { min: 1, max: 100 };

    let board_create_settings = BoardCreateSettings {
        branching_factor: 3,
    };
    let seed: u64 = 1;

    let rng = rand::SeedableRng::seed_from_u64(seed);

    let boards = board_create_settings
        .create_boards::<3, 3>(solve_settings, rng)
        .take(number_of_boards);

    for board in boards {
        let solutions_len = solve_settings.solve(board).count();

        assert_eq!(100, solutions_len);
    }
}
