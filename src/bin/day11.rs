struct ProblemState {
    galaxies: Galaxies,
}

impl ProblemState {
    fn new(extend: usize) -> Self {
        let input = include_str!("../../input.txt")
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let cols: Vec<usize> = (0..input[0].len())
            .filter(|&i| input.iter().all(|line| line[i] != '#'))
            .collect();
        let rows: Vec<usize> = (0..input.len())
            .filter(|&i| input[i].iter().all(|&ch| ch != '#'))
            .collect();

        let mut galaxies = Galaxies::new();
        for (r, row) in input.iter().enumerate() {
            for (c, &ch) in row.iter().enumerate() {
                if ch == '#' {
                    let new_r = r + rows.iter().filter(|&&rx| r > rx).count() * (extend - 1);
                    let new_c = c + cols.iter().filter(|&&cx| c > cx).count() * (extend - 1);

                    galaxies.add_galaxy(Galaxy::new(new_c as i64, new_r as i64));
                }
            }
        }

        Self { galaxies }
    }
}

struct Galaxy {
    x: i64,
    y: i64,
}

impl Galaxy {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
    fn path(&self, other: &Galaxy) -> i64 {
        (other.x - self.x).abs() + (other.y - self.y).abs()
    }
}

struct Galaxies {
    elements: Vec<Galaxy>,
}

impl Galaxies {
    fn new() -> Self {
        Self { elements: vec![] }
    }
    fn add_galaxy(&mut self, galaxy: Galaxy) {
        self.elements.push(galaxy);
    }
    fn shortest_distance(&self) -> Vec<i64> {
        let mut result = vec![];

        for i in 0..self.elements.len() {
            for j in i..self.elements.len() {
                let first = &self.elements[i];
                let second = &self.elements[j];

                let distance = first.path(second);
                result.push(distance);
            }
        }

        result
    }
}

fn main() {
    let state1 = ProblemState::new(2);
    let state2 = ProblemState::new(1_000_000);
    println!("Part 1: {}", part1(&state1));
    println!("Part 2: {}", part1(&state2));
}

fn part1(state: &ProblemState) -> String {
    state
        .galaxies
        .shortest_distance()
        .iter()
        .sum::<i64>()
        .to_string()
}
