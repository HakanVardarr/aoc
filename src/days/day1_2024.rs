use crate::{day, Error, InputReciever, Solver};
use std::iter::zip;

fn generate_groups(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut first_group: Vec<i32> = Vec::new();
    let mut second_group: Vec<i32> = Vec::new();

    for line in input.lines() {
        let values = line.split_once(" ").unwrap();
        first_group.push(values.0.trim().parse().unwrap());
        second_group.push(values.1.trim().parse().unwrap());
    }

    (first_group, second_group)
}

day!(Day1_2024, 1, 2024);
impl Solver for Day1_2024 {
    fn first_part(&self) -> String {
        let (mut first_group, mut second_group) = generate_groups(&self.input);
        first_group.sort();
        second_group.sort();

        let mut result: u32 = 0;
        for numbers in zip(first_group, second_group) {
            result += numbers.0.abs_diff(numbers.1);
        }

        result.to_string()
    }
    fn second_part(&self) -> String {
        let (first_group, second_group) = generate_groups(&self.input);

        let mut result = 0;
        first_group.iter().for_each(|num| {
            result += num * second_group.iter().filter(|&x| x == num).count() as i32;
        });

        result.to_string()
    }
}
