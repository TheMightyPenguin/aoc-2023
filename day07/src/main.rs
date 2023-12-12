use std::collections::HashMap;
use std::env::current_dir;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

// (contents, kind, values, bid)
type Hand = (String, HandKind, Vec<i64>, i64);

fn main() {
    // (hand, bid)
    let mut hands: Vec<Hand> = vec![];
    let lines = read_lines("./src/input.txt")
        .expect("File should exist")
        .map(|l| l.expect("Line should be readable"))
        .collect::<Vec<_>>();

    for line in lines {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let values = parts[0]
            .chars()
            .map(|c| get_card_value(c))
            .collect::<Vec<_>>();

        hands.push((
            String::from(parts[0]),
            get_hand_kind(String::from(parts[0])),
            values,
            parts[1].parse::<i64>().expect("bid should be a number"),
        ));
    }

    hands.sort_by(|a, b| {
        let (_a_content, a_kind, a_vals, _a_bid) = a;
        let (_b_content, b_kind, b_vals, _b_bid) = b;
        if a_kind != b_kind {
            return a_kind.cmp(&b_kind);
        }
        let mut index = 0;
        while index < a_vals.len() {
            if a_vals[index] != b_vals[index] {
                return a_vals[index].cmp(&b_vals[index]);
            }
            index += 1;
        }
        return 1.cmp(&0);
    });

    let mut sum = 0;

    for (index, hand) in hands.iter().enumerate() {
        let bid = hand.3;
        sum += bid * (index as i64 + 1);
    }

    println!("Sum: {}", sum);
}

fn get_hand_kind(hand_content: String) -> HandKind {
    let mut seen_cards: HashMap<char, usize> = HashMap::new();

    for card in hand_content.chars() {
        let card_count = seen_cards.entry(card).or_insert(0);
        *card_count += 1;
    }

    let values = seen_cards.values().collect::<Vec<_>>();
    let non_joker_values = seen_cards
        .iter()
        .filter(|(key, _)| **key != 'J')
        .map(|(_, value)| *value)
        .collect::<Vec<_>>();

    let multiplied_values = seen_cards.values().fold(1, |acc, x| acc * x);
    let joker_count = *seen_cards.get(&'J').unwrap_or(&0);

    let has_4_non_j_equals = non_joker_values.contains(&&4);
    let has_3_non_j_equals = non_joker_values.contains(&&3);
    let has_2_non_j_equals = non_joker_values.contains(&&2);

    if values.contains(&&5)
        || (has_4_non_j_equals && joker_count == 1)
        || (has_3_non_j_equals && joker_count == 2)
        || (has_2_non_j_equals && joker_count == 3)
        || (joker_count == 4)
    {
        return HandKind::FiveOfAKind;
    }

    if values.contains(&&4)
        || (has_3_non_j_equals && joker_count == 1)
        || (has_2_non_j_equals && joker_count == 2)
        || (joker_count == 3)
    {
        return HandKind::FourOfAKind;
    }

    if multiplied_values == 6 || (multiplied_values == 4 && joker_count == 1) {
        return HandKind::FullHouse;
    }

    if values.contains(&&3) || (has_2_non_j_equals && joker_count == 1) || (joker_count == 2) {
        return HandKind::ThreeOfAKind;
    }

    if multiplied_values == 4 {
        return HandKind::TwoPairs;
    }

    if multiplied_values == 2 || joker_count == 1 {
        return HandKind::OnePair;
    }

    HandKind::HighCard
}

fn get_card_value(card: char) -> i64 {
    match card {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'T' => 10,
        'J' => 1,
        _ => card.to_digit(10).unwrap() as i64,
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let dir = current_dir()?;
    let file_path = dir.join(filename);
    let file = File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn getting_hand_kind() {
        assert_eq!(get_hand_kind(String::from("AAAAA")), HandKind::FiveOfAKind);
        assert_eq!(get_hand_kind(String::from("AA8AA")), HandKind::FourOfAKind);
        assert_eq!(get_hand_kind(String::from("23332")), HandKind::FullHouse);
        assert_eq!(get_hand_kind(String::from("TTT98")), HandKind::ThreeOfAKind);
        assert_eq!(get_hand_kind(String::from("23432")), HandKind::TwoPairs);
        assert_eq!(get_hand_kind(String::from("A23A4")), HandKind::OnePair);
        assert_eq!(get_hand_kind(String::from("23456")), HandKind::HighCard);
    }

    #[test]
    fn getting_hand_kind_with_jokers() {
        assert_eq!(get_hand_kind(String::from("JJJJJ")), HandKind::FiveOfAKind);
        assert_eq!(get_hand_kind(String::from("AAAAJ")), HandKind::FiveOfAKind);
        assert_eq!(get_hand_kind(String::from("JJAJJ")), HandKind::FiveOfAKind);
        assert_eq!(get_hand_kind(String::from("AAAJJ")), HandKind::FiveOfAKind);
        assert_eq!(get_hand_kind(String::from("AAAJ3")), HandKind::FourOfAKind);
        assert_eq!(get_hand_kind(String::from("333JA")), HandKind::FourOfAKind);
        assert_eq!(get_hand_kind(String::from("JJ3JA")), HandKind::FourOfAKind);
        assert_eq!(get_hand_kind(String::from("33J22")), HandKind::FullHouse);
        assert_eq!(get_hand_kind(String::from("AA3J2")), HandKind::ThreeOfAKind);
        assert_eq!(get_hand_kind(String::from("12JJ4")), HandKind::ThreeOfAKind);
        assert_eq!(get_hand_kind(String::from("33JA2")), HandKind::ThreeOfAKind);
        assert_eq!(get_hand_kind(String::from("123J4")), HandKind::OnePair);
    }
}
