use advent_of_lib::{day, Solver};

fn calculate_difference(numbers: &Vec<i32>) -> Vec<i32> {
    let differences = &numbers
        .iter()
        .enumerate()
        .map(|(i, _n)| {
            if i < numbers.len() - 1 {
                numbers[i + 1] - numbers[i]
            } else {
                numbers[i]
            }
        })
        .collect::<Vec<i32>>()[..numbers.len() - 1];

    differences.to_vec()
}

fn check_differences(differences: &Vec<i32>) -> bool {
    for i in 0..differences.len() - 1 {
        let j = differences[i];
        let k = differences[i + 1];

        if j.abs() > 3 || j == 0 || k.abs() > 3 || k == 0 || (j < 0 && k > 0) || (j > 0 && k < 0) {
            return false;
        }
    }

    return true;
}

day!(Day2, 2, 2024);
impl Solver for Day2 {
    fn first_part(&self) -> String {
        let mut result: u32 = 0;
        for line in self.input.lines() {
            let numbers: Vec<i32> = line.split(" ").map(|s| s.parse::<i32>().unwrap()).collect();
            if check_differences(&calculate_difference(&numbers)) {
                result += 1;
            }
        }
        result.to_string()
    }
    fn second_part(&self) -> String {
        let mut result: u32 = 0;

        for line in self.input.lines() {
            let numbers: Vec<i32> = line.split(" ").map(|s| s.parse::<i32>().unwrap()).collect();
            let mut is_valid = check_differences(&calculate_difference(&numbers));

            if !is_valid {
                for i in 0..numbers.len() {
                    let mut numbers_clone = numbers.clone();
                    numbers_clone.remove(i);

                    is_valid = is_valid || check_differences(&calculate_difference(&numbers_clone));
                }
            }

            if is_valid {
                result += 1;
            }
        }

        result.to_string()
    }
}
