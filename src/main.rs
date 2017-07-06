// This is O(1) space and O(N^2) time complexity
fn naive_solution(vec: &Vec<i32>) -> i32 {
    fn get_left_highest_point(vec: &Vec<i32>, current_index : usize) -> i32 {
        let mut current_highest_point = &vec[0];

        for x in 1..current_index {
            if current_highest_point < &vec[x] {
                current_highest_point = &vec[x]
            }
        }

        return *current_highest_point;
    }

    fn get_right_highest_point(vec : &Vec<i32>, current_index : usize) -> i32 {
        let mut current_highest_point = &vec[vec.len() - 1];

        for x in current_index..vec.len() - 1 {
            if current_highest_point < &vec[x] {
                current_highest_point = &vec[x]
            }
        }

        return *current_highest_point;
    }

    fn get_stored_rain(left : i32, right : i32, current : i32) -> i32 {
        if left < right {
            return left - current;
        } else {
            return right - current;
        }
    }

    let mut total_stored_rain = 0;

    for x in 0..vec.len() {
        let current_elevation = *&vec[x];
        let left_highest_point = get_left_highest_point(&vec, x);
        if left_highest_point <= current_elevation { continue; }
        let right_highest_point = get_right_highest_point(&vec, x);
        if right_highest_point <= current_elevation { continue; }
        total_stored_rain += get_stored_rain(left_highest_point, right_highest_point, current_elevation);
    }

    return total_stored_rain;
}

fn main() {
    println!("This is the rainfall problem");
    println!("Basically, you want to calculate the amount of water than can be stored between two mountains");
    println!("An array will represent the mountains height :");
    let vec = vec![4, 5, 3, 2, 3, 6, 3, 2, 4, 3];
    println!("{:?}", &*vec);

    println!("Stored rain : {}", naive_solution(&vec));
}
