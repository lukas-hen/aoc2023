use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;


#[allow(dead_code)]
#[allow(unused)]
pub fn part_1() -> Result<(), Box<dyn Error>> {

    let filepath = "data/day_04/1_real.in".to_string();
    let file = File::open(&filepath)?;
    let reader = BufReader::new(file);

    let mut cards: Vec<String> = vec![];

    for line in reader.lines() {
        cards.push(line?
            .split(": ")
            .last()
            .unwrap()
            .to_string()
        );
    }

    let mut rewards: Vec<u32> = vec![];

    for card in cards {
        let split: Vec<&str> = card.split("|").collect();
        let (win_str, your_str) = (split.first().unwrap(), split.last().unwrap());
        let win_nums: Vec<u32> = win_str.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect();
        let your_nums: Vec<u32> = your_str.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect();
       
        let win_set: HashSet<u32> = HashSet::from_iter(win_nums.iter().cloned());
        let your_set: HashSet<u32> = HashSet::from_iter(your_nums.iter().cloned());
        let intersect: Vec<&u32> = win_set.intersection(&your_set).into_iter().collect();
        let num_matches: u32 = intersect.len() as u32;

        rewards.push(reward_from_n(num_matches));
    }

    let reward_sum: u32 = rewards.iter().sum();

    println!("{}", reward_sum);

    Ok(())
}

fn reward_from_n(n: u32) -> u32 {
    match n {
        0 => 0,
        _ => 2_u32.pow(n - 1),
    }
}