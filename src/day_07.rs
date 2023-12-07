use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::str::FromStr;
use std::cmp::Eq;
use core::cmp::Ordering;

pub fn part_1_and_2() -> Result<(), Box<dyn Error>> {
    let file_path = "data/day_07/1_real.in".to_string();
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut hands_and_bid: Vec<(Hand, u32)> = vec![];

    for line in reader.lines() {
        let l = line?;
        let mut split = l.split_whitespace();
        let hand = split.next().unwrap();
        let bid = split.last().unwrap();

        hands_and_bid.push(
            (
                hand.parse::<Hand>()?, 
                bid.parse::<u32>()?,
            )
        );
    }

    // asc
    hands_and_bid.sort_by(|(h1, _), (h2, _)| h1.cmp(h2));

    let mut multiplier = 1;
    let mut accumulated_score = 0;

    for (hand, bid) in hands_and_bid {
        accumulated_score += multiplier*bid;
        multiplier += 1;
    }

    println!("{}", accumulated_score);

    Ok(())
}

#[derive(PartialEq, Hash, Eq, PartialOrd, Clone, Copy, Debug)]
enum CardValue {
    // Sorted by ascending value.
    // J was moved to top in part 2
    J, 
    Two,
    Three,
    Four, 
    Five, 
    Six, 
    Seven, 
    Eight, 
    Nine, 
    T, 
    Q, 
    K, 
    A, 
}

impl CardValue {

    fn from_char(c: &char) -> Result<Self, String> {
        match c {
            // J was moved to top in part 2
            'J' => Ok(CardValue::J),
            '2' => Ok(CardValue::Two),
            '3' => Ok(CardValue::Three),
            '4' => Ok(CardValue::Four),
            '5' => Ok(CardValue::Five),
            '6' => Ok(CardValue::Six),
            '7' => Ok(CardValue::Seven),
            '8' => Ok(CardValue::Eight),
            '9' => Ok(CardValue::Nine),
            'T' => Ok(CardValue::T),
            'Q' => Ok(CardValue::Q),
            'K' => Ok(CardValue::K),
            'A' => Ok(CardValue::A),
            e => Err(format!("Couldn't parse char: '{}' to CardValue.", e)),
        }        
    }
}

#[derive(
    Debug, 
    Eq, 
    PartialEq, 
    Ord, 
    PartialOrd, 
    Copy, 
    Clone
)]
enum HandCombination {
    // Sorted by ascending value.
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(
    Debug, 
    Eq, 
    PartialEq, 
    Clone,
)]
struct Hand {
    combination: HandCombination,
    cards: Vec<CardValue>,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let combination_ordering = self.combination.cmp(&other.combination);
        
        if combination_ordering.is_eq() {
            // Maybe would be good to check hand sizes match here.
            let mut card_ordering = Ordering::Equal;
            for card_idx in 0..self.cards.len() {
                if self.cards[card_idx] < other.cards[card_idx] {
                    card_ordering = Ordering::Less;
                    break;
                } else if self.cards[card_idx] > other.cards[card_idx] {
                    card_ordering = Ordering::Greater;
                    break;
                }
            }

            return card_ordering
        }
        
        combination_ordering
    }
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = s.chars();
        
        Ok(Hand{
            combination: s.parse::<HandCombination>()?,
            cards: c.map(|c| CardValue::from_char(&c).unwrap()).collect(),
        })
    }
}

impl FromStr for HandCombination {

    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input_len = s.len();

        if input_len != 5 {
            return Err("A HandCombination has to be of length 5.".to_string())
        }

        let mut value_map: HashMap<CardValue, i8> = HashMap::new();

        for c in s.chars() {
            let value = CardValue::from_char(&c)?;
            let count = value_map.entry(value).or_insert(0);
            *count += 1;
        }
    
        let joker_count = value_map.get(&CardValue::J).unwrap_or(&0).clone();
        value_map.remove_entry(&CardValue::J);

        // Safeguard.
        // If all jokers, the logic below will not work as we have no other cards to 
        // add the joker count to. Unwrap would probably panic somewhere without the guard.
        if joker_count == input_len as i8 {
            return Ok(HandCombination::FiveOfAKind)
        }
        
        let mut l: Vec<i8> = value_map
            .values()
            .map(|v| v.clone())
            .collect();

        l.sort(); // asc

        // Most efficient use of jokers is always to add them to the already highest num.
        let mut last = l.last_mut().unwrap();
        *last += joker_count;

        match l.as_slice() {
            [1, 1, 1, 1, 1] => Ok(HandCombination::HighCard),
            [1, 1, 1, 2] => Ok(HandCombination::OnePair),
            [1, 2, 2] => Ok(HandCombination::TwoPair),
            [1, 1, 3] => Ok(HandCombination::ThreeOfAKind),
            [2, 3] => Ok(HandCombination::FullHouse),
            [1, 4] => Ok(HandCombination::FourOfAKind),
            [5] => Ok(HandCombination::FiveOfAKind),
            _ => Err("Unknown HandCombination pattern.".to_string()),
        }
    }

}