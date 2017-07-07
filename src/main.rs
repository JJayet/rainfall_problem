#[macro_use]
extern crate timeit;

mod naive_solution;
mod better_solution;
mod betterer_solution;

use naive_solution::naive_solution::NaiveSolution;
use better_solution::better_solution::BetterSolution;
use betterer_solution::betterer_solution::BettererSolution;

fn main() {
    println!("This is the rainfall problem");
    println!("Basically, you want to calculate the amount of water than can be stored between two mountains");
    println!("An array will represent the mountains height :");
    let vec = vec![4, 5, 3, 2, 3, 6, 3, 2, 4, 3, 5, 324, 3, 320];
    println!("{:?}", vec);

    let naive_solution = NaiveSolution {
        vec : vec.clone()
    };

    timeit!({
        //println!("Stored rain in naive solution : {}", naive_solution.compute());
        assert!(337 == naive_solution.compute())
    });

    let mut better_solution = BetterSolution::new(vec.clone());
    timeit!({
        //println!("Stored rain in better solution : {}", better_solution.compute());
        assert!(337 == better_solution.compute())
    });

    let mut betterer_solution = BettererSolution::new(vec.clone());
    timeit!({
        //println!("Stored rain in betterer solution : {}", betterer_solution.compute());
        assert!(337 == betterer_solution.compute())
    });   
}
