mod naive_solution;
mod better_solution;

use naive_solution::naive_solution::NaiveSolution;
use better_solution::better_solution::BetterSolution;

fn main() {
    println!("This is the rainfall problem");
    println!("Basically, you want to calculate the amount of water than can be stored between two mountains");
    println!("An array will represent the mountains height :");
    let vec = vec![4, 5, 3, 2, 3, 6, 3, 2, 4, 3];
    println!("{:?}", vec);

    let naive_solution = NaiveSolution {
        vec : vec.clone()
    };

    println!("Stored rain in naive solution : {}", naive_solution.compute());

    let mut better_solution = BetterSolution::new(vec.clone());

    println!("Stored rain in better solution : {}", better_solution.compute());
}
