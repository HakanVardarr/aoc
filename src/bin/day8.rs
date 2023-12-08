struct ProblemState {}

impl ProblemState {
    fn new() -> Self {
        let input = include_str!("../../inputs/sample8.txt")
            .lines()
            .collect::<Vec<_>>();

        let mut instructions = Vec::new();
        input[0].chars().for_each(|c| match c {
            'R' => instructions.push(Direction::Right),
            'L' => instructions.push(Direction::Left),
            _ => (),
        });

        println!("{instructions:?}");

        Self {}
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

struct Element {
    left: Box<Element>,
    right: Box<Element>,
}

fn main() {
    let state = ProblemState::new();

    println!("Part 1: {}", part1(&state));
    println!("-------------------");
    println!("Part 2: {}", part2(&state));
}

fn part1(state: &ProblemState) -> String {
    String::from("Unsolved")
}

fn part2(state: &ProblemState) -> String {
    String::from("Unsolved")
}
