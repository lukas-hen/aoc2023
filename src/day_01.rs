use std::fs::File;
use std::error::Error;
use std::io::{BufReader, BufRead};

#[allow(dead_code)]

pub fn run() -> Result<(), Box<dyn Error>> {
    
    let filepath = String::from("data/day_01/1_real.in");
    let file = File::open(&filepath)?;
    let reader = BufReader::new(file);

    let mut all_fl_digits: Vec<String> = vec![];

    for line in reader.lines() {
       let nums = translate_text_to_num(&line?);
       let first_last = get_first_last_digits(&nums);
       all_fl_digits.push(first_last);
    }

    let sum: i64 = all_fl_digits.iter()
        .map(|d| str::parse::<i64>(d).unwrap())
        .sum();

    println!("{}", sum);

    Ok(())
}


fn get_first_last_digits(s: &String) -> String {

    let str_nums: String = s
        .chars()
        .filter(|c| c.is_numeric())
        .collect();

    let first = str_nums.chars().next().unwrap();
    let last = str_nums.chars().last().unwrap();

    format!("{}{}", first, last)
    
}

fn translate_text_to_num(s: &String) -> String {
    let len = s.len();
    let mut str_out = String::from("");
    for i in 0..len {
        let curr_char = s.chars().nth(i).unwrap();
        if curr_char.is_numeric() {
            str_out.push_str(&String::from(curr_char));
        }
        str_out.push_str(&parse_digit_name(&s[i..len]));
    }
    str_out
}


fn parse_digit_name(s: &str) -> String { 
    match s { 
        _ if s.starts_with("one") => String::from("1"),
        _ if s.starts_with("two") => String::from("2"),
        _ if s.starts_with("three") => String::from("3"),
        _ if s.starts_with("four") => String::from("4"),
        _ if s.starts_with("five") => String::from("5"),
        _ if s.starts_with("six") => String::from("6"),
        _ if s.starts_with("seven") => String::from("7"),
        _ if s.starts_with("eight") => String::from("8"),
        _ if s.starts_with("nine") => String::from("9"),
        _ => String::from(""),
    }
}