#[derive(Debug, Default)]
struct ProblemState {
    seeds: Vec<u64>,
    map: Vec<Vec<Vec<u64>>>,
}

impl ProblemState {
    fn parse(&mut self) {
        let input = include_str!("../../inputs/day5.txt")
            .split("\n")
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>();

        self.seeds = input[1]
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        let mut map = vec![];
        let mut i = 2;
        let mut current_vec = vec![];

        if (input[i].as_bytes()[0] as char).is_ascii_alphabetic() {}

        while i < input.len() - 1 {
            i += 1;

            if (input[i].as_bytes()[0] as char).is_ascii_alphabetic() {
                map.push(current_vec);
                current_vec = vec![];
            }

            if (input[i].as_bytes()[0] as char).is_ascii_digit() {
                current_vec.push(
                    input[i]
                        .split_whitespace()
                        .map(|x| x.parse::<u64>().unwrap())
                        .collect::<Vec<_>>(),
                )
            }
        }

        map.push(current_vec);
        self.map = map;
    }
}

fn main() {
    let mut state = ProblemState::default();
    state.parse();

    println!("Part 1: {}", part1(&mut state));
    println!("--------------------");
    println!("Part 2: {}", part2());
}

fn part1(state: &mut ProblemState) -> String {
    for seed in state.seeds.iter_mut() {
        'next_map: for i in 0..state.map.len() {
            let mut changed = false;
            for s in &state.map[i] {
                if changed {
                    continue 'next_map;
                }
                let i = s[1];
                let k = i + s[2];

                if *seed >= i && *seed <= k {
                    *seed = *seed - i + s[0];
                    changed = true
                }
            }
        }
    }

    state.seeds.iter().min().unwrap().to_string()
}

fn part2() -> String {
    String::from("Unsolved")
}
