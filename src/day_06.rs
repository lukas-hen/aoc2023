use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead, Lines};
use core::str::FromStr;

pub fn part_1() -> Result<(), Box<dyn Error>> {
    let filepath = "data/day_06/1_real.in".to_string();
    let file = File::open(&filepath)?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    let time_vec: Vec<i64> = parse_line(&lines.next().unwrap()?);
    let record_vec: Vec<i64> = parse_line(&lines.next().unwrap()?);

    let mut n_breaking = vec![];

    for i in 0..time_vec.len() {
        n_breaking.push(get_hold_times_breaking_record(&time_vec[i], &record_vec[i]).len());
    }

    let product = n_breaking
        .iter()
        .copied()
        .reduce(|acc, e| acc*e)
        .unwrap();
    
    println!("{:?}", product);

    Ok(())
}

pub fn part_2() -> Result<(), Box<dyn Error>> {
    let filepath = "data/day_06/1_real.in".to_string();
    let file = File::open(&filepath)?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    let time = parse_line_part_2(&lines.next().unwrap()?);
    let record = parse_line_part_2(&lines.next().unwrap()?); 

    let num_record_times = get_hold_times_breaking_record(&time, &record).len();

    println!("{}", num_record_times);

    Ok(())
}

fn parse_line(l: &String) -> Vec<i64> {
    l.split_whitespace()
        .skip(1) // First element is measure name.
        .map(|s| s.parse::<i64>().unwrap())
        .collect()       
}

fn parse_line_part_2(l: &String) -> i64 {
    let s = l
        .split_whitespace()
        .skip(1) // First element is measure name.
        .collect::<String>();
    
    s.parse::<i64>().unwrap()
}

fn get_hold_times_breaking_record(max_time: &i64, record_dist: &i64) -> Vec<i64> {

    // d = distance, h = hold_time, t = total_time
    // Distance follows: d = h*(t-h) -> d = ht-h^2
    // this is always a parabola - meaning that ALL 
    // record times between the smallest hold time giving distance > record
    // and biggest hold time that gives distance > record 
    // are guaranteed to beat the record aswell.

    fn calc_distance(hold_time: &i64, max_time: &i64) -> i64 {
        hold_time*(max_time - hold_time)
    }

    let hold_times = 0..*max_time;
    let mut lower_bound = 0;
    let mut upper_bound = 0;
    
    for hold_time in hold_times.clone() {
        if calc_distance(&hold_time, max_time) > *record_dist {
            lower_bound = hold_time;
            break;
        }
    }

    // reverse iterator, doesn't reverse in memory.
    for hold_time in hold_times.rev() { 
        if calc_distance(&hold_time, max_time) > *record_dist {
            upper_bound = hold_time;
            break;
        }
    }

    // Inclusive upper bound
    (lower_bound..=upper_bound).collect()
}