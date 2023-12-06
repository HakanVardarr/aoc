#[derive(Debug)]
struct ProblemState {
    times: Vec<u64>,
    distances: Vec<u64>,
}

impl ProblemState {
    fn new() -> Self {
        let input = include_str!("../../inputs/day6.txt");
        let lines = input.lines().collect::<Vec<_>>();

        Self {
            times: lines[0]
                .split_whitespace()
                .skip(1)
                .map(|x| x.parse().unwrap())
                .collect(),
            distances: lines[1]
                .split_whitespace()
                .skip(1)
                .map(|x| x.parse().unwrap())
                .collect(),
        }
    }
}

fn main() {
    let state = ProblemState::new();

    println!("Part 1: {}", part1(&state));
    println!("---------------------");
    println!("Part 2: {}", part2(&state));
}

fn part1(state: &ProblemState) -> String {
    state
        .times
        .iter()
        .zip(&state.distances)
        .fold(1, |acc, (&time, &record)| {
            acc * (0..time).filter(|&t| t * (time - t) > record).count()
        })
        .to_string()
}

fn part2(state: &ProblemState) -> String {
    let time = state
        .times
        .iter()
        .map(|&x| x.to_string())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    let distance = state
        .distances
        .iter()
        .map(|&x| x.to_string())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    let mut total_record_broken = 0;
    for t in 0..time {
        let dt = time - t;
        let dx = t * dt;
        if dx > distance {
            total_record_broken += 1;
        }
    }

    total_record_broken.to_string()
}
