use advent_of_lib::{day, Solver};
use rayon::prelude::*;

#[derive(Debug)]
struct Equation {
    answer: usize,
    numbers: Vec<usize>,
}

impl Equation {
    fn new(line: &str) -> Self {
        let (answer, numbers) = line.split_once(":").unwrap();
        let answer = answer.parse::<usize>().unwrap();
        let numbers = numbers
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        Self { answer, numbers }
    }

    fn check(&self, part_2: bool) -> bool {
        let mut expresions = vec![
            (
                format!("{}+{}", self.numbers[0], self.numbers[1]),
                self.numbers[0] + self.numbers[1],
            ),
            (
                format!("{}*{}", self.numbers[0], self.numbers[1]),
                self.numbers[0] * self.numbers[1],
            ),
        ];
        if part_2 {
            expresions.push((
                format!("{}||{}", self.numbers[0], self.numbers[1]),
                format!("{}{}", self.numbers[0], self.numbers[1])
                    .parse::<usize>()
                    .unwrap(),
            ));
        }

        for i in 2..self.numbers.len() {
            let mut new_expresions = vec![];
            for expr in &expresions {
                for op in ["+", "*", "||"] {
                    if op == "||" && !part_2 {
                        continue;
                    }

                    let eq = format!("{}{}{}", expr.0, op, self.numbers[i]);
                    let ans = match op {
                        "+" => expr.1 + self.numbers[i],
                        "*" => expr.1 * self.numbers[i],
                        "||" => format!("{}{}", expr.1, self.numbers[i])
                            .parse::<usize>()
                            .unwrap(),
                        _ => unreachable!(),
                    };

                    new_expresions.push((eq, ans));
                }
            }
            expresions = new_expresions;
        }

        for (_, answer) in expresions {
            if answer == self.answer {
                return true;
            }
        }

        false
    }
}

day!(Day7, 7, 2024);
impl Solver for Day7 {
    fn first_part(&self) -> String {
        self.input
            .par_lines()
            .filter_map(|line| {
                let equation = Equation::new(line);
                if equation.check(false) {
                    return Some(equation.answer);
                }
                None
            })
            .sum::<usize>()
            .to_string()
    }

    fn second_part(&self) -> String {
        self.input
            .par_lines()
            .filter_map(|line| {
                let equation = Equation::new(line);
                if equation.check(true) {
                    return Some(equation.answer);
                }
                None
            })
            .sum::<usize>()
            .to_string()
    }
}
