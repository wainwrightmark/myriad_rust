#[macro_use]
extern crate bencher;

use bencher::Bencher;
use myriad::core::prelude::*;

benchmark_group!(benches, bench_find_solutions);
benchmark_main!(benches);

fn bench_find_solutions(bench: &mut Bencher) {
    bench.iter(|| create_boards_and_solve(1));
}

fn create_boards_and_solve(number_of_boards: usize) {
    let solve_settings = SolveSettings { min: 1, max: 100 };

    let board_create_settings = BoardCreateSettings {
        branching_factor: 3,
        number_to_return: number_of_boards,
    };
    let rng = rand::SeedableRng::seed_from_u64(100);
    let rng_cell = core::cell::RefCell::new(rng);

    let boards = &create_boards(solve_settings, 9, &board_create_settings, &rng_cell);

    for board in boards {        
        let solutions = solve_settings.solve(board)            
            .collect::<Vec<FoundWord>>();

        assert_eq!(100, solutions.len());
    }
}
