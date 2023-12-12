use std::collections::HashSet;

fn main() {
    let lines = include_str!("../../input.txt").lines().collect::<Vec<_>>();
    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: HashSet<(i64, i64)> = HashSet::new();
    let mut gears: HashSet<(i64, i64)> = HashSet::new();

    let mut num: Option<Number> = None;
    for (row, line) in lines.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                if let Some(ref mut number) = num {
                    number.add_digit(row as i64, col as i64, c);
                } else {
                    num = Some(Number::new(row as i64, col as i64, c));
                }
            } else {
                if let Some(number) = num.take() {
                    numbers.push(number);
                }

                if c != '.' {
                    symbols.insert((row as i64, col as i64));
                    if c == '*' {
                        gears.insert((row as i64, col as i64));
                    }
                }
            }
        }
    }

    println!("Part 1: {}", part1(&numbers, &symbols));
    println!("Part 2: {}", part2(&numbers, &gears));
}

#[derive(Debug)]
struct Number {
    value: i64,
    points: HashSet<(i64, i64)>,
}

impl Number {
    fn new(row: i64, col: i64, c: char) -> Self {
        Self {
            value: (c as u8 - b'0') as i64,
            points: HashSet::from([
                (row - 1, col - 1),
                (row, col - 1),
                (row + 1, col - 1),
                (row - 1, col),
                (row + 1, col),
                (row - 1, col + 1),
                (row, col + 1),
                (row + 1, col + 1),
            ]),
        }
    }
    fn add_digit(&mut self, row: i64, col: i64, c: char) {
        self.value = self.value * 10 + (c as u8 - b'0') as i64;
        self.points
            .extend([(row + 1, col + 1), (row, col + 1), (row - 1, col + 1)])
    }
}

fn part1(numbers: &Vec<Number>, symbols: &HashSet<(i64, i64)>) -> String {
    let mut sum = 0;
    for number in numbers {
        for symbol in symbols {
            if number.points.contains(symbol) {
                sum += number.value;
            }
        }
    }

    sum.to_string()
}

fn part2(numbers: &Vec<Number>, gears: &HashSet<(i64, i64)>) -> String {
    let mut sum = 0;
    for gear in gears {
        let mut matches = Vec::new();
        for num in numbers {
            if num.points.contains(gear) {
                matches.push(num.value);
            }
        }
        if matches.len() == 2 {
            sum += matches[0] * matches[1];
        }
    }

    sum.to_string()
}
