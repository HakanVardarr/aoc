use std::collections::HashMap;

struct ProblemState<'a> {
    directions: Vec<Direction>,
    elements: HashMap<&'a str, (&'a str, &'a str)>,
}

impl<'a> ProblemState<'a> {
    fn new() -> Self {
        let input = include_str!("../../inputs/day8.txt")
            .lines()
            .collect::<Vec<_>>();

        let directions: Vec<Direction> = input[0]
            .chars()
            .filter(|&ch| ch == 'R' || ch == 'L')
            .map(|ch| match ch {
                'R' => Direction::Right,
                'L' => Direction::Left,
                _ => unreachable!(),
            })
            .collect();

        let mut elements: HashMap<&str, (&str, &str)> = HashMap::new();
        for &element in input.iter().skip(2) {
            let inner_elements = element.split(" = ").collect::<Vec<_>>();
            let element = inner_elements[0];
            let map = inner_elements[1].split(",").collect::<Vec<_>>();
            let left = std::str::from_utf8(&map[0].trim().as_bytes()[1..]).unwrap();
            let right = std::str::from_utf8(&map[1].trim().as_bytes()[..3]).unwrap();

            elements.insert(element, (left, right));
        }

        Self {
            directions,
            elements,
        }
    }

    fn lcm_of_vector(numbers: &[u64]) -> Option<u64> {
        if numbers.is_empty() {
            None
        } else {
            let result = numbers
                .iter()
                .fold(numbers[0], |acc, &x| ProblemState::lcm(acc, x));
            Some(result)
        }
    }

    fn lcm(a: u64, b: u64) -> u64 {
        if a == 0 || b == 0 {
            return 0;
        } else {
            return a / ProblemState::gcd(a, b) * b;
        }
    }

    fn gcd(mut a: u64, mut b: u64) -> u64 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }

        a
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

fn main() {
    let state = ProblemState::new();

    println!("Part 1: {}", part1(&state));
    println!("-------------------");
    println!("Part 2: {}", part2(&state));
}

fn part1(state: &ProblemState) -> String {
    let mut current = "AAA";
    let mut step = 0;
    'outter: loop {
        for direction in state.directions.iter() {
            step += 1;
            let current_element = state.elements.get(current).unwrap();
            match &direction {
                Direction::Left => current = current_element.0,
                Direction::Right => current = current_element.1,
            }

            if current == "ZZZ" {
                break 'outter;
            }
        }
    }

    step.to_string()
}

fn part2(state: &ProblemState) -> String {
    let mut start_positions = state
        .elements
        .iter()
        .filter(|(&element, _)| element.ends_with("A"))
        .map(|(&element, _)| element)
        .collect::<Vec<_>>();

    let end_positions = state
        .elements
        .iter()
        .filter(|(&element, _)| element.ends_with("Z"))
        .map(|(&element, _)| element)
        .collect::<Vec<_>>();

    let mut total_step = 0;
    let mut steps = vec![];
    for mut current in start_positions {
        let mut step = 0;
        'outter: loop {
            for direction in state.directions.iter() {
                step += 1;
                let current_element = state.elements.get(current).unwrap();
                match &direction {
                    Direction::Left => current = current_element.0,
                    Direction::Right => current = current_element.1,
                }

                if end_positions.contains(&current) {
                    break 'outter;
                }
            }
        }
        steps.push(step);
    }

    total_step = ProblemState::lcm_of_vector(&steps).unwrap();
    total_step.to_string()
}
