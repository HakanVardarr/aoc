struct ProblemState {
    numbers: Vec<Vec<Vec<u64>>>,
}

impl ProblemState {
    fn new() -> Self {
        ProblemState { numbers: vec![] }
    }
    fn parse(&mut self) {
        let input = include_str!("../../inputs/day4.txt")
            .lines()
            .collect::<Vec<_>>();

        for line in &input {
            let card_numbers = line.split(":").collect::<Vec<_>>()[1]
                .trim()
                .split("|")
                .collect::<Vec<_>>();

            self.numbers.push(
                card_numbers
                    .iter()
                    .map(|vec| {
                        vec.split(" ")
                            .filter(|x| x.parse::<u64>().is_ok())
                            .map(|x| x.parse::<u64>().unwrap())
                            .collect::<Vec<_>>()
                    })
                    .collect(),
            );
        }
    }
}

fn main() {
    let mut problem = ProblemState::new();
    problem.parse();

    println!("Part 1: {}", part1(&problem));
    println!("----------------------------");
    println!("Part 2: {}", part2(&problem));
}

fn part1(state: &ProblemState) -> String {
    let mut sum = 0;
    for line in &state.numbers {
        let winning_numbers = &line[0];
        let numbers = &line[1];
        let mut current: Option<u64> = None;
        for number in numbers {
            if winning_numbers.contains(number) {
                if let Some(_) = current {
                    current = Some(current.unwrap() * 2);
                } else {
                    current = Some(1);
                }
            }
        }
        if current.is_some() {
            sum += current.unwrap();
        }
    }

    sum.to_string()
}

fn part2(state: &ProblemState) -> String {
    let mut result_cards = vec![1; state.numbers.len()];

    for (array_index, line) in state.numbers.iter().enumerate() {
        let winning_numbers = &line[0];
        let numbers = &line[1];

        let mut current: Option<u64> = None;
        for number in numbers {
            if winning_numbers.contains(number) {
                if let Some(_) = current {
                    current = Some(current.unwrap() + 1);
                } else {
                    current = Some(1);
                }
            }
        }

        for i in 1..current.unwrap_or_default() + 1 {
            let index = array_index + i as usize;
            if index < result_cards.len() {
                result_cards[index] += result_cards[array_index];
            }
        }
    }

    result_cards.iter().sum::<u64>().to_string()
}
