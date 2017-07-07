// This is O(N) space and O(N) time complexity
#[derive(Debug)]
pub struct BetterSolution {
    vec: Vec<i32>,
    left_vec: Vec<i32>,
    right_vec: Vec<i32>,
}

impl BetterSolution {
    fn calculate_left_highest_points(&mut self) {
        let mut current_highest_point = 0;
        for (i, &item) in self.vec.iter().enumerate() {
            if item > current_highest_point {
                current_highest_point = item
            }
            self.left_vec[i] = current_highest_point;
        }
    }

    fn calculate_right_highest_points(&mut self) {
        let mut current_highest_point = self.vec[self.vec.len() - 1];

        for i in (0..self.vec.len()).rev() {
            if current_highest_point < self.vec[i] {
                current_highest_point = self.vec[i]
            }
            self.right_vec[i] = current_highest_point;
        }
    }

    fn get_stored_rain(&self, left: i32, right: i32, current: i32) -> i32 {
        if left < right {
            left - current
        } else {
            right - current
        }
    }

    pub fn compute(&mut self) -> i32 {
        let mut total_stored_rain = 0;

        self.calculate_left_highest_points();
        self.calculate_right_highest_points();

        for (index, &current_item) in self.vec.iter().enumerate() {
            total_stored_rain +=
                self.get_stored_rain(self.left_vec[index], self.right_vec[index], current_item);
        }

        total_stored_rain
    }

    pub fn new(vec: Vec<i32>) -> BetterSolution {
        let length = vec.capacity();
        let mut bs = BetterSolution {
            vec: vec,
            left_vec: Vec::with_capacity(length),
            right_vec: Vec::with_capacity(length),
        };

        unsafe {
            bs.left_vec.set_len(length);
            bs.right_vec.set_len(length);
        }

        bs
    }
}
