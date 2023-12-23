use std::{cmp::Ordering, collections::HashMap, ops::Mul};

fn card_to_strength(card: char) -> Option<usize> {
    match card {
        'A' => Some(13),
        'K' => Some(12),
        'Q' => Some(11),
        'T' => Some(10),
        '9' => Some(9),
        '8' => Some(8),
        '7' => Some(7),
        '6' => Some(6),
        '5' => Some(5),
        '4' => Some(4),
        '3' => Some(3),
        '2' => Some(2),
        '1' => Some(1),
        'J' => Some(0),
        _ => None,
    }
}

fn get_card_counts(hand: &str) -> HashMap<char, usize> {
    hand.chars().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    })
}

fn get_hand_type(hand: &str) -> Option<usize> {
    let mut card_frequency = get_card_counts(hand);
    let jokers = card_frequency.remove_entry(&'J');
    let c = card_frequency.clone();
    let max = c.iter().max_by_key(|entry| entry.1);

    match jokers {
        Some((_, jv)) => match max {
            Some((mk, _)) => match card_frequency.get_mut(mk) {
                Some(mv) => *mv += jv,
                None => (),
            },
            None => (),
        },
        None => (),
    }

    let values = card_frequency.values().collect::<Vec<_>>();
    match card_frequency.len() {
        1 | 0 => Some(6), // five of a kind
        2 => {
            if values.contains(&&4) {
                return Some(5); // four of a kind
            }
            return Some(4); // full house
        }
        3 => {
            if values.contains(&&3) {
                return Some(3); // three of a kind
            };
            return Some(2); // two pair
        }
        4 => Some(1), // one pair
        5 => Some(0), // high card
        _ => None,
    }
}

fn compare_types(h1: &str, h2: &str) -> Ordering {
    let h1_type = get_hand_type(h1).unwrap();
    let h2_type = get_hand_type(h2).unwrap();
    if h1_type < h2_type {
        return Ordering::Less;
    }
    if h1_type > h2_type {
        return Ordering::Greater;
    }
    return Ordering::Equal;
}

fn compare_cards(h1: &str, h2: &str) -> Ordering {
    for (c1, c2) in h1.chars().zip(h2.chars()) {
        if card_to_strength(c1) < card_to_strength(c2) {
            return Ordering::Less;
        }
        if card_to_strength(c1) > card_to_strength(c2) {
            return Ordering::Greater;
        }
    }
    return Ordering::Equal;
}

fn compare_hands(h1: &str, h2: &str) -> Ordering {
    match compare_types(h1, h2) {
        Ordering::Equal => compare_cards(h1, h2),
        v => v,
    }
}

fn main() {
    let mut hb = include_str!("../input.txt")
        .lines()
        .map(|line| {
            let mut s = line.split_whitespace();
            (
                s.next().unwrap(),
                s.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<(&str, usize)>>();
    hb.sort_by(|a, b| compare_hands(a.0, b.0));
    println!(
        "{:#?}",
        hb.iter()
            .enumerate()
            .map(|(i, (_hand, bid))| bid.mul(i + 1))
            .sum::<usize>()
    )
}
