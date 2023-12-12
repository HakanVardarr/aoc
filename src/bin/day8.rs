use std::collections::HashMap;

struct ProblemState<'a> {
    directions: &'a str,
    elements: HashMap<&'a str, (&'a str, &'a str)>,
}

impl<'a> ProblemState<'a> {
    fn new() -> Self {
        let input = include_str!("../../input.txt").lines().collect::<Vec<_>>();

        let directions = input[0];

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

fn main() {
    let state = ProblemState::new();

    println!("Part 1: {}", part1(&state));
    println!("-------------------");
    println!("Part 2: {}", part2(&state));
}

fn part1(state: &ProblemState) -> String {
    let mut current = "AAA";

    for (s, direction) in state.directions.chars().cycle().enumerate() {
        let current_element = state.elements.get(current).unwrap();
        match direction {
            'L' => current = current_element.0,
            'R' => current = current_element.1,
            _ => (),
        }

        if current == "ZZZ" {
            return s.to_string();
        }
    }

    unreachable!()
}

fn part2(state: &ProblemState) -> String {
    let start_positions = state
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

    let mut steps = vec![];
    for mut current in start_positions {
        for (s, direction) in state.directions.chars().cycle().enumerate() {
            if end_positions.contains(&current) {
                steps.push(s as u64);
                break;
            }
            let current_element = state.elements.get(current).unwrap();
            match &direction {
                'L' => current = current_element.0,
                'R' => current = current_element.1,
                _ => (),
            }
        }
    }

    let total_step = ProblemState::lcm_of_vector(&steps).unwrap();
    total_step.to_string()
}
