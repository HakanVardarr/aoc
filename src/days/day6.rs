use rayon::prelude::*;
use std::collections::HashMap;

use advent_of_lib::{day, Solver};

#[derive(Debug, Clone, PartialEq)]
struct Obstacle {
    x: usize,
    y: usize,
}

impl Obstacle {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

impl Direction {
    fn new(c: char) -> Self {
        match c {
            '>' => Self::RIGHT,
            '<' => Self::LEFT,
            '^' => Self::UP,
            'v' => Self::DOWN,
            _ => unreachable!(),
        }
    }

    fn step(&self, position: &mut (usize, usize), n: isize) {
        match self {
            Self::DOWN => *position = (position.0, (position.1 as isize + n) as usize),
            Self::UP => *position = (position.0, (position.1 as isize - n) as usize),
            Self::LEFT => *position = ((position.0 as isize - n) as usize, position.1),
            Self::RIGHT => *position = ((position.0 as isize + n) as usize, position.1),
        }
    }

    fn rotate(&mut self) {
        match self {
            Direction::DOWN => *self = Direction::LEFT,
            Direction::UP => *self = Direction::RIGHT,
            Direction::LEFT => *self = Direction::UP,
            Direction::RIGHT => *self = Direction::DOWN,
        }
    }
}

#[derive(Debug)]
struct Guard {
    x: usize,
    y: usize,
    direction: Direction,
}

impl Guard {
    fn new(x: usize, y: usize, direction: Direction) -> Self {
        Self { x, y, direction }
    }

    fn route(
        &self,
        obstacles: &Vec<Obstacle>,
        boundaries: (usize, usize),
    ) -> Option<Vec<(usize, usize)>> {
        let mut position = (self.x, self.y);
        let mut direction = self.direction;

        let mut result: Vec<(usize, usize)> = Vec::new();
        let mut visited: HashMap<(usize, usize), usize> = HashMap::new();

        while position.0 > 0
            && position.0 < boundaries.0
            && position.1 > 0
            && position.1 < boundaries.1
        {
            direction.step(&mut position, 1);

            if obstacles.contains(&Obstacle::new(position.0, position.1)) {
                direction.step(&mut position, -1);
                direction.rotate();
            } else {
                if let Some(count) = visited.get_mut(&position) {
                    *count += 1;

                    if *count > 4 {
                        return None;
                    }
                } else {
                    result.push(position);
                    visited.insert(position, 1);
                }
            }
        }

        Some(result)
    }
}

fn generate_obstacles_and_guard(
    input: String,
    boundaries: &mut (usize, usize),
) -> (Vec<Obstacle>, Guard) {
    let mut obstacles = Vec::new();
    let mut guard = Guard::new(0, 0, Direction::LEFT);

    let lines = input.lines().collect::<Vec<_>>();
    *boundaries = (lines.len(), lines[0].len());

    for (y, &line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => obstacles.push(Obstacle::new(x, y)),
                '^' | '<' | '>' | 'v' => guard = Guard::new(x, y, Direction::new(c)),
                _ => {}
            }
        }
    }

    (obstacles, guard)
}

day!(Day6, 6, 2024);
impl Solver for Day6 {
    fn first_part(&self) -> String {
        let mut boundaries = (0, 0);
        let (obstacles, guard) = generate_obstacles_and_guard(self.input.clone(), &mut boundaries);

        let route = guard.route(&obstacles, boundaries).unwrap();
        route.len().to_string()
    }

    fn second_part(&self) -> String {
        let mut boundaries = (0, 0);
        let (obstacles, guard) = generate_obstacles_and_guard(self.input.clone(), &mut boundaries);

        guard
            .route(&obstacles, boundaries)
            .unwrap()
            .par_iter()
            .filter(|(x, y)| {
                if *x == guard.x && *y == guard.y {
                    return false;
                } else {
                    let mut obstacles = obstacles.clone();
                    obstacles.push(Obstacle::new(*x, *y));

                    guard.route(&obstacles, boundaries).is_none()
                }
            })
            .count()
            .to_string()
    }
}
