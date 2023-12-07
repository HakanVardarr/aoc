use std::collections::HashMap;

struct ProblemState {
    hands: Hands,
}

impl ProblemState {
    fn new(part2: bool) -> Self {
        let input = include_str!("../../inputs/day7.txt");
        let lines = input.lines().collect::<Vec<_>>();

        let mut hands = Hands::new();
        for line in lines {
            let list = line.split(" ").collect::<Vec<_>>();
            let hand = Hand::new(list[0].into(), list[1].parse::<u64>().unwrap(), part2);

            hands.add(hand, part2);
        }

        hands.list_hands();
        Self { hands }
    }
}

struct Hands(Vec<Hand>);

impl Hands {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn add(&mut self, mut hand: Hand, part2: bool) {
        hand.find_win_type(part2);
        self.0.push(hand);
    }

    fn list_hands(&mut self) {
        loop {
            let mut swapped = false;

            for i in 0..self.0.len() - 1 {
                if self.0[i].win_type > self.0[i + 1].win_type {
                    self.0.swap(i, i + 1);
                    swapped = true;
                } else if self.0[i].win_type == self.0[i + 1].win_type {
                    let first = &self.0[i].cards.clone();
                    let second = &self.0[i + 1].cards.clone();

                    'inner: for k in 0..5 {
                        let first_card = &first[k];
                        let second_card = &second[k];

                        if first_card > second_card {
                            self.0.swap(i, i + 1);
                            swapped = true;
                            break 'inner;
                        } else if first_card < second_card {
                            break 'inner;
                        }
                    }
                }
            }

            if !swapped {
                break;
            }
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash)]
struct Hand {
    cards: Vec<CardType>,
    bid: u64,
    win_type: Option<WinType>,
}

impl Hand {
    fn new(cards: String, bid: u64, part2: bool) -> Self {
        let mut parsed_cards = Vec::new();

        for card in cards.chars() {
            match card {
                '2' => parsed_cards.push(CardType::Two),
                '3' => parsed_cards.push(CardType::Three),
                '4' => parsed_cards.push(CardType::Four),
                '5' => parsed_cards.push(CardType::Five),
                '6' => parsed_cards.push(CardType::Six),
                '7' => parsed_cards.push(CardType::Seven),
                '8' => parsed_cards.push(CardType::Eight),
                '9' => parsed_cards.push(CardType::Nine),
                'T' => parsed_cards.push(CardType::Ten),
                'J' => {
                    if part2 {
                        parsed_cards.push(CardType::Joker)
                    } else {
                        parsed_cards.push(CardType::J)
                    }
                }
                'Q' => parsed_cards.push(CardType::Q),
                'K' => parsed_cards.push(CardType::K),
                'A' => parsed_cards.push(CardType::A),
                _ => (),
            }
        }

        Self {
            cards: parsed_cards,
            bid,
            win_type: None,
        }
    }

    fn find_win_type(&mut self, part2: bool) {
        let mut label_count: HashMap<CardType, u8> = HashMap::new();

        for label in &self.cards {
            if let Some(count) = label_count.get_mut(&label) {
                *count += 1;
            } else {
                label_count.insert((*label).clone(), 1);
            }
        }

        let mut count = 0;
        if part2 {
            if label_count.get(&CardType::Joker).is_some() {
                count = label_count.remove(&CardType::Joker).unwrap();
            }
        }

        let mut sorted_count = label_count.iter().collect::<Vec<_>>();
        sorted_count.sort_by(|(_, &a), (_, &b)| b.cmp(&a));

        if part2 {
            if sorted_count.len() == 0 {
                self.win_type = Some(WinType::FiveOfAKind);
            }
        }

        for (i, (_, &v)) in sorted_count.iter().enumerate() {
            let mut v = v;
            if part2 {
                if i == 0 {
                    v += count;
                }
            }

            if v == 5 {
                self.win_type = Some(WinType::FiveOfAKind);
                break;
            } else if v == 4 {
                self.win_type = Some(WinType::FourOfAKind);
                break;
            } else if v == 3 {
                self.win_type = Some(WinType::ThreeOfAKind);
            } else if v == 2 {
                if let Some(win_type) = &self.win_type {
                    if *win_type == WinType::ThreeOfAKind {
                        self.win_type = Some(WinType::FullHouse);
                        break;
                    } else if *win_type == WinType::OnePair {
                        self.win_type = Some(WinType::TwoPair);
                        break;
                    }
                } else {
                    self.win_type = Some(WinType::OnePair);
                }
            } else {
                if self.win_type.is_none() {
                    self.win_type = Some(WinType::HighCard);
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Clone)]
enum CardType {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    J,
    Q,
    K,
    A,
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd)]
enum WinType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn main() {
    let state_part1 = ProblemState::new(false);
    let state_part2 = ProblemState::new(true);

    println!("Part 1: {}", part1(&state_part1));
    println!("--------------------");
    println!("Part 2: {}", part2(&state_part2));
}

fn part1(state: &ProblemState) -> String {
    state
        .hands
        .0
        .iter()
        .enumerate()
        .fold(0, |acc: u64, (index, hand)| {
            acc + hand.bid * (index as u64 + 1)
        })
        .to_string()
}

fn part2(state: &ProblemState) -> String {
    state
        .hands
        .0
        .iter()
        .enumerate()
        .fold(0, |acc: u64, (index, hand)| {
            acc + hand.bid * (index as u64 + 1)
        })
        .to_string()
}
