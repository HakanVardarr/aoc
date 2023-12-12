use std::collections::VecDeque;

struct ProblemState {
    histories: Vec<History>,
}

impl ProblemState {
    fn new() -> Self {
        let input = include_str!("../../input.txt").lines().collect::<Vec<_>>();

        let mut histories = vec![];

        for history in input {
            let mut differences = vec![];
            let values = history
                .split_ascii_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<VecDeque<_>>();
            differences.push(values.clone());

            let mut current_value = values;
            loop {
                let mut temp = VecDeque::new();
                for i in 0..current_value.len() - 1 {
                    temp.push_back(current_value[i + 1] - current_value[i])
                }
                current_value = temp.clone();
                differences.push(temp);

                if current_value.iter().all(|&x| x == 0) {
                    break;
                }
            }

            let mut history = History::new(differences);
            history.genereate_next_sequence();
            histories.push(history);
        }

        Self { histories }
    }
}

#[derive(Debug)]
struct History {
    values: Vec<VecDeque<i64>>,
}

impl History {
    fn new(values: Vec<VecDeque<i64>>) -> Self {
        Self { values }
    }
    fn genereate_next_sequence(&mut self) {
        let difference_count = self.values.len();
        for i in (0..difference_count).rev() {
            if i == difference_count - 1 {
                self.values[i].push_back(0);
                continue;
            }

            let index = self.values[i].len() - 1;
            let value_back = self.values[i][index] + self.values[i + 1][index];
            let value_front = self.values[i][0] - self.values[i + 1][0];

            self.values[i].push_back(value_back);
            self.values[i].push_front(value_front);
        }
    }
}

fn main() {
    let state = ProblemState::new();
    println!("Part 1: {}", part1(&state));
    println!("Part 2: {}", part2(&state));
}

fn part1(state: &ProblemState) -> String {
    state
        .histories
        .iter()
        .fold(0, |acc, x| acc + x.values[0].iter().last().unwrap())
        .to_string()
}

fn part2(state: &ProblemState) -> String {
    state
        .histories
        .iter()
        .fold(0, |acc, x| acc + x.values[0][0])
        .to_string()
}
