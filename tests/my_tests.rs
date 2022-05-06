#[cfg(test)]

use std::cell::RefCell;

use myriad::core::board::*;
use myriad::core::solver::*;
use myriad::core::creator::*;


#[test]
    fn from_string_test() {
        test_board("98_-7+524", 100);
    }
    
    
    fn test_board(letters: &str, expected_count: usize)
    {
        let board = Board::try_create(letters).expect("board should be created");
        
        let solver = Solver{settings: SolveSettings{min:1, max:100}};
    
        let solutions = solver.get_possible_solutions(&board).collect::<Vec<FoundWord>>();
    
        // let solution_nums = solutions.iter().map(|x|x.result).sorted().join(",");
        // eprintln!("{}",solution_nums);
    
        assert_eq!(expected_count, solutions.len());
    }
    
    #[test]
    fn test_create_boards()
    {
        let solver = Solver{settings: SolveSettings{min:1, max:100}};
    
    
        let settings = BoardCreateSettings{
            branches_to_take: 2,
            desired_solutions: 100,
            number_to_return: 10
        };
        let rng = rand::SeedableRng::seed_from_u64(100);
        let rng_cell = RefCell::new(rng);
    
        let boards = &myriad::core::creator::create_boards(&solver, 9, &settings, &rng_cell);
    
    
    
        for board in boards{
    
            eprintln!();
            eprintln!("{}",board.to_multiline_string());
    
            eprintln!();
        }
    
        assert!(boards.len() >= settings.number_to_return);
    
    }
