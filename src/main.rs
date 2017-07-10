#[macro_use]
extern crate timeit;

mod naive_solution;
mod better_solution;
mod betterer_solution;

use naive_solution::NaiveSolution;
use better_solution::BetterSolution;
use betterer_solution::BettererSolution;

fn main() {
    println!("This is the rainfall problem");
    println!("Basically, you want to calculate the amount of water than can be stored between two mountains");
    println!("An array will represent the mountains height :");
    let vec = vec![4, 5, 3, 2, 3, 6, 3, 2, 4, 3, 5, 324, 3, 320];
    println!("{:?}", vec);

    let naive_solution = NaiveSolution::new(&vec);
    let mut better_solution = BetterSolution::new(&vec);
    let mut betterer_solution = BettererSolution::new(&vec);
    
    let number_of_loops = 100000;

    let print_result = |sec : f64, solution : &str| {
        let (mult, unit_str) = if sec > 1.0 {
            (1.0, "s")
        } else if sec > 0.001 {
            (0.001, "ms")
        } else if sec > 0.000_001 {
            (0.000_001, "µs")
        } else {
            (0.000_000_001, "ns")
        };

        println!("{} loops of {} solution : {} {}", number_of_loops, solution, sec / mult, unit_str)
    };

    let time = timeit_loops!(number_of_loops, {
        assert!(337 == naive_solution.compute())
    });

    print_result(time, "naive");

    let time = timeit_loops!(number_of_loops, {
        assert!(337 == better_solution.compute())
    });

    print_result(time, "better");

    let time = timeit_loops!(number_of_loops, {
        assert!(337 == betterer_solution.compute())
    });   

    print_result(time, "betterer");
}
