#[derive(Debug, Default)]
struct ProblemState {
    seeds: Vec<u64>,
    map: Vec<Vec<Vec<u64>>>,
}

impl ProblemState {
    fn parse(&mut self) {
        let input = include_str!("../../input.txt")
            .split("\n")
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>();

        self.seeds = input[0].split(":").collect::<Vec<_>>()[1]
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        self.map = input[2..]
            .split(|line| line.chars().next().unwrap().is_ascii_alphabetic())
            .filter(|group| !group.is_empty())
            .map(|group| {
                group
                    .iter()
                    .map(|line| {
                        line.split_whitespace()
                            .map(|x| x.parse::<u64>().unwrap())
                            .collect()
                    })
                    .collect()
            })
            .collect();
    }
}

fn main() {
    let mut state1 = ProblemState::default();
    let mut state2 = ProblemState::default();
    state1.parse();
    state2.parse();

    println!("Part 1: {}", part1(&mut state1));
    println!("--------------------");
    println!("Part 2: {}", part2(&mut state2));
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

fn part2(state: &mut ProblemState) -> String {
    let mut reverse = 1;
    loop {
        let mut curent_reverse = reverse;
        'next_map: for i in (0..state.map.len()).rev() {
            let mut changed = false;
            for s in &state.map[i] {
                if changed {
                    continue 'next_map;
                }
                let i = s[0];
                let k = i + s[2] - 1;

                if curent_reverse >= i && curent_reverse <= k {
                    curent_reverse = curent_reverse + s[1] - s[0];
                    changed = true
                }
            }
        }
        for seeds in state.seeds.chunks(2) {
            if curent_reverse >= seeds[0] && curent_reverse < seeds[1] + seeds[0] {
                return (reverse).to_string();
            }
        }
        reverse += 1;
    }
}
