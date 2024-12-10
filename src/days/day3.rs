use advent_of_lib::{day, Solver};
use regex::Regex;

day!(Day3, 3, 2024);
impl Solver for Day3 {
    fn first_part(&self) -> String {
        let mut result: u32 = 0;
        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

        for (_, [num1, num2]) in re.captures_iter(&self.input).map(|c| c.extract()) {
            result += num1.parse::<u32>().unwrap() * num2.parse::<u32>().unwrap();
        }

        result.to_string()
    }

    fn second_part(&self) -> String {
        let mut result: u32 = 0;
        let re = Regex::new(r"mul\((\d+),(\d+)\)|(do\(\))|(don't\(\))").unwrap();

        let mut mul = true;
        for capture in re.captures_iter(&self.input) {
            if let Some(mul_match) = capture.get(1) {
                if mul {
                    result += mul_match.as_str().parse::<u32>().unwrap()
                        * capture.get(2).unwrap().as_str().parse::<u32>().unwrap();
                }
            } else if capture.get(3).is_some() {
                mul = true;
            } else if capture.get(4).is_some() {
                mul = false;
            }
        }

        result.to_string()
    }
}
