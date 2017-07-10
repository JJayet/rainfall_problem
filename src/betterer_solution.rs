// This is O(N) space and O(N) time complexity
#[derive(Debug)]
pub struct BettererSolution<'a> {
    vec: &'a Vec<i32>,
    diff_vec: Vec<i32>,
}

impl<'a> BettererSolution<'a> {
    fn calculate_left_highest_points(&mut self) {
        let mut current_highest_point = 0;
        for (i, &item) in self.vec.iter().enumerate() {
            if item > current_highest_point {
                current_highest_point = item
            }
            self.diff_vec[i] = current_highest_point
        }
    }

    fn calculate_right_highest_points(&mut self) {
        let mut current_highest_point = 0;

        for i in (0..self.vec.len()).rev() {
            if current_highest_point < self.vec[i] {
                current_highest_point = self.vec[i]
            }
            if current_highest_point < self.diff_vec[i] {
                self.diff_vec[i] = current_highest_point
            }
        }

    }

    fn get_stored_rain(&self, diff: i32, current: i32) -> i32 {
        diff - current
    }

    pub fn compute(&mut self) -> i32 {
        let mut total_stored_rain = 0;

        self.calculate_left_highest_points();
        self.calculate_right_highest_points();

        for (index, &current_item) in self.vec.iter().enumerate() {
            total_stored_rain += self.get_stored_rain(self.diff_vec[index], current_item);
        }

        total_stored_rain
    }

    pub fn new(vec: &'a Vec<i32>) -> BettererSolution {
        let length = vec.capacity();
        let mut bs = BettererSolution {
            vec: vec,
            diff_vec: Vec::with_capacity(length),
        };

        unsafe { bs.diff_vec.set_len(length) }

        bs
    }
}
