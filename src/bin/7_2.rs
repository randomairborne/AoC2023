use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let mut sum = 0;
    let mut hands = Vec::new();
    for line in input.lines() {
        let split_line: Vec<&str> = line.splitn(2, ' ').collect();
        let hand = split_line[0];
        let bid: usize = split_line[1].parse().unwrap();
        hands.push((Hand::new(hand), bid));
    }
    hands.sort_by(|lhs, rhs| lhs.0.cmp(&rhs.0));
    for (rank, (hand, bid)) in hands.iter().enumerate() {
        println!("{} {hand:?} {:?} {bid}", rank + 1, hand.kind());
        sum += bid * (rank + 1)
    }
    println!("{sum}");
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Hand {
    values: [Card; 5],
}

impl Hand {
    fn kind(&self) -> HandType {
        let mut joker_positions = Vec::with_capacity(5);
        for idx in 0..5 {
            if self.values[idx] == Card::Joker {
                joker_positions.push(idx);
            }
        }
        if joker_positions.is_empty() {
            self.concrete_kind()
        } else {
            let mut max = HandType::High;
            for card in Card::ALL_NORMAL {
                let mut values = self.values.clone();
                values[joker_positions[0]] = card;
                let kind = Self { values }.kind();
                if kind > max {
                    max = kind;
                }
            }
            max
        }
    }
    fn concrete_kind(&self) -> HandType {
        let mut totals = HashMap::new();
        for card in Card::ALL_NORMAL {
            totals.insert(card, 0);
        }
        for value in &self.values {
            *totals.get_mut(value).unwrap() += 1;
        }
        let mut twos_seen = 0;
        let mut three_seen = false;
        for count in totals.values() {
            let count = *count;
            match count {
                2 => twos_seen += 1,
                3 => three_seen = true,
                4 => return HandType::Four,
                5 => return HandType::Five,
                _ => {}
            }
        }
        if twos_seen >= 1 && three_seen {
            HandType::FullHouse
        } else if three_seen {
            HandType::Three
        } else if twos_seen >= 2 {
            HandType::TwoPair
        } else if twos_seen >= 1 {
            HandType::Pair
        } else {
            HandType::High
        }
    }
    fn new(data: &str) -> Self {
        let data_chars: Vec<char> = data.chars().collect();
        let mut values = [Card::Ace; 5];
        for (idx, value) in values.iter_mut().enumerate() {
            *value = Card::from_char(data_chars[idx]);
        }
        Self { values }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.kind().cmp(&other.kind()) {
            Ordering::Equal => {}
            val => return val,
        }
        self.values.cmp(&other.values)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
enum HandType {
    High,
    Pair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
enum Card {
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
    Queen,
    King,
    Ace,
}

impl Card {
    pub const ALL_NORMAL: [Self; 12] = [
        Self::Ace,
        Self::King,
        Self::Queen,
        Self::Ten,
        Self::Nine,
        Self::Eight,
        Self::Seven,
        Self::Six,
        Self::Five,
        Self::Four,
        Self::Three,
        Self::Two,
    ];
    fn from_char(data: char) -> Self {
        match data {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Joker,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => panic!("Unexpected character {data}"),
        }
    }
}
