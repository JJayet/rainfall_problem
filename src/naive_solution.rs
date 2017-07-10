// This is O(1) space and O(N^2) time complexity
#[derive(Debug)]
pub struct NaiveSolution<'a> {
    pub vec: &'a Vec<i32>,
}

impl<'a> NaiveSolution<'a> {
    pub fn new(vec: &'a Vec<i32>) -> NaiveSolution {
        NaiveSolution { vec: vec }
    }

    fn get_left_highest_point(&self, current_index: usize) -> i32 {
        let mut current_highest_point = self.vec[0];

        for x in 1..current_index {
            if current_highest_point < self.vec[x] {
                current_highest_point = self.vec[x]
            }
        }

        current_highest_point
    }

    fn get_right_highest_point(&self, current_index: usize) -> i32 {
        let mut current_highest_point = self.vec[self.vec.len() - 1];

        for x in current_index..self.vec.len() - 1 {
            if current_highest_point < self.vec[x] {
                current_highest_point = self.vec[x]
            }
        }

        current_highest_point
    }

    fn get_stored_rain(&self, left: i32, right: i32, current: i32) -> i32 {
        if left < right {
            left - current
        } else {
            right - current
        }
    }

    pub fn compute(&self) -> i32 {
        let mut total_stored_rain = 0;

        for (index, &current_elevation) in self.vec.iter().enumerate() {
            let left_highest_point = self.get_left_highest_point(index);
            if left_highest_point <= current_elevation {
                continue;
            }
            let right_highest_point = self.get_right_highest_point(index);
            if right_highest_point <= current_elevation {
                continue;
            }
            total_stored_rain +=
                self.get_stored_rain(left_highest_point, right_highest_point, current_elevation);
        }

        total_stored_rain
    }
}
