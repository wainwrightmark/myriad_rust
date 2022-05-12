#[macro_use]
extern crate bencher;

use bencher::Bencher;
use myriad::core::prelude::*;

benchmark_group!(benches, bench_find_solutions);
benchmark_main!(benches);

fn bench_find_solutions(bench: &mut Bencher) {
    bench.iter(|| create_boards_and_solve(10));
}

fn create_boards_and_solve(number_of_boards: usize) {
    let solve_settings = SolveSettings { min: 1, max: 100 };

    let board_create_settings = BoardCreateSettings {
        branching_factor: 3,
    };
    let seed: u64 = rand::random();

    let rng = rand::SeedableRng::seed_from_u64(seed);

    let boards = board_create_settings.create_boards(9, solve_settings, rng)
    .take(number_of_boards);

    for board in boards {        
        let solutions = solve_settings.solve(board)            
            .collect::<Vec<FoundWord>>();

        assert_eq!(100, solutions.len());
    }
}
