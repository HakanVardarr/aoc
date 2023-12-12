use std::collections::HashMap;

struct ProblemState {
    input: Vec<Vec<char>>,
}

impl ProblemState {
    fn new() -> Self {
        let input = include_str!("../../input.txt")
            .lines()
            .map(|x| x.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Self { input }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Position {
    y: i64,
    x: i64,
    value: char,
}

impl Position {
    fn new(y: i64, x: i64, value: char) -> Self {
        Self { y, x, value }
    }
    fn start_pos(input: &Vec<Vec<char>>) -> Option<Position> {
        for (i, k) in input.iter().enumerate() {
            if let Some(pos) = k.iter().position(|&x| x == 'S') {
                return Some(Position::new(i as i64, pos as i64, input[i][pos]));
            }
        }

        None
    }
    fn get_adjacent_grounds(&self, input: &Vec<Vec<char>>) -> Vec<Position> {
        let sides = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        let mut adjacent_positions = Vec::new();

        for side in sides {
            if self.y + side.0 >= 0
                && self.y + side.0 < input.len() as i64
                && self.x + side.1 >= 0
                && self.x + side.1 < input[0].len() as i64
            {
                let y = (self.y + side.0) as usize;
                let x = (self.x + side.1) as usize;
                if input[y][x] == '.' {
                    adjacent_positions.push(Position::new(y as i64, x as i64, input[y][x]))
                }
            }
        }

        adjacent_positions
    }

    fn step(
        &self,
        input: &Vec<Vec<char>>,
        step: usize,
        visited: &mut Vec<Vec<bool>>,
        grounds: &mut HashMap<Position, u8>,
        part2: bool,
    ) -> usize {
        let mut max_distance = step;
        let able_to_go = match &self.value {
            '|' => vec![(0, 1), (0, -1)],
            '-' => vec![(1, 0), (-1, 0)],
            'L' => vec![(1, 0), (0, -1)],
            'J' => vec![(-1, 0), (0, -1)],
            'F' => vec![(1, 0), (0, 1)],
            '7' => vec![(-1, 0), (0, 1)],
            'S' => vec![(1, 0), (-1, 0), (0, 1), (0, -1)],
            _ => vec![(0, 0)],
        };

        for go in able_to_go {
            let x = self.x + go.0;
            let y = self.y + go.1;

            if y >= 0
                && y < input.len() as i64
                && x >= 0
                && x < input[0].len() as i64
                && input[y as usize][x as usize] != '.'
                && !visited[y as usize][x as usize]
            {
                let position = Position::new(y, x, input[y as usize][x as usize]);

                if position.value == 'S' {
                    max_distance = max_distance.max(step);
                }

                if part2 {
                    for ground in position.get_adjacent_grounds(input) {
                        *grounds.get_mut(&ground).unwrap() += 1;
                    }
                }

                visited[y as usize][x as usize] = true;
                let result = position.step(input, step + 1, visited, grounds, part2);
                max_distance = max_distance.max(result)
            }
        }

        max_distance
    }
}

fn main() {
    let state = ProblemState::new();

    println!("Part 1: {}", part1(&state));
    println!("Part 2: {}", part2(&state));
}

fn part1(state: &ProblemState) -> String {
    let mut visited = vec![vec![false; state.input[0].len()]; state.input.len()];
    let mut grounds = HashMap::new();
    for (i, x) in state.input.iter().enumerate() {
        for (k, c) in x.iter().enumerate() {
            if c == &'.' {
                grounds.insert(Position::new(i as i64, k as i64, '.'), 0);
            }
        }
    }

    let start_pos = Position::start_pos(&state.input).unwrap();
    (start_pos.step(&state.input, 1, &mut visited, &mut grounds, false) / 2).to_string()
}

fn part2(state: &ProblemState) -> String {
    let mut visited = vec![vec![false; state.input[0].len()]; state.input.len()];
    let mut grounds = HashMap::new();
    for (i, x) in state.input.iter().enumerate() {
        for (k, c) in x.iter().enumerate() {
            if c == &'.' {
                grounds.insert(Position::new(i as i64, k as i64, '.'), 0);
            }
        }
    }

    let start_pos = Position::start_pos(&state.input).unwrap();
    let _ = start_pos.step(&state.input, 1, &mut visited, &mut grounds, true) / 2;

    String::from("Unsolved")
}
