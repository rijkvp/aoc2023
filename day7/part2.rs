use std::{
    cmp::Ordering,
    collections::HashMap,
    io::{stdin, BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    bid: u64,
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(' ').unwrap();
        Ok(Self {
            cards: cards.chars().collect(),
            bid: bid.parse().unwrap(),
        })
    }
}

impl Hand {
    fn hand_type(&self) -> u64 {
        let mut map = HashMap::<char, usize>::new();
        for card in &self.cards {
            map.entry(*card).and_modify(|c| *c += 1).or_insert(1);
        }
        let jokers = map.remove(&'J');
        let mut card_types: Vec<usize> = map.values().map(|c| *c).collect();
        card_types.sort_by(|a, b| b.cmp(&a));
        if let Some(jokers) = jokers {
            if let Some(d) = card_types.get_mut(0) {
                *d += jokers;
            } else {
                card_types.push(jokers);
            }
        }
        if card_types.len() == 1 {
            6 // Five of a kind
        } else if card_types[0] == 4 {
            5 // Full house
        } else if card_types[0] == 3 {
            if card_types[1] == 2 {
                4 // Full house
            } else {
                3 // Three of a kind
            }
        } else if card_types.len() == 3 {
            2 // Two pair
        } else if card_types.len() == 4 {
            1 // One pair
        } else {
            0 // High card
        }
    }
}

fn card_strength(card: char) -> u64 {
    if card.is_alphabetic() {
        return match card {
            'A' => 13,
            'K' => 12,
            'Q' => 11,
            'T' => 10,
            'J' => 1,
            _ => unreachable!(),
        };
    }
    card.to_digit(10).unwrap() as u64
}

fn compare_hands(a: &Hand, b: &Hand) -> Ordering {
    let c1 = a.hand_type().cmp(&b.hand_type());
    if c1 == Ordering::Equal {
        for i in 0..5 {
            let c2 = card_strength(a.cards[i]).cmp(&card_strength(b.cards[i]));
            if c2 != Ordering::Equal {
                return c2;
            }
        }
        return Ordering::Equal;
    }
    c1
}

fn main() {
    let mut hands = BufReader::new(stdin())
        .lines()
        .map(|l| Hand::from_str(&l.unwrap()).unwrap())
        .collect::<Vec<Hand>>();
    hands.sort_by(compare_hands);
    let mut winnings = 0;
    for (i, hand) in hands.into_iter().enumerate() {
        winnings += hand.bid * (i + 1) as u64;
    }
    println!("{winnings:?}")
}
