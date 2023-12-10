use std::fmt::Debug;
use std::error::Error;
use std::fs::File;
use std::io::{ BufReader, BufRead };

pub fn part_1_and_2() -> Result<(), Box<dyn Error>> {

    let file_path = "data/day_09/1_real.in".to_string();
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut p1_total = 0;
    let mut p2_total = 0;
    
    for line in reader.lines() {
        let l = line?;
        let nums = l
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        p1_total += get_next_number(&nums);
        p2_total += get_prev_number(&nums);
            
    }

    println!("p1 solution: {}", p1_total);
    println!("p2 solution: {}", p2_total);

    Ok(())
}

fn get_next_number(series: &Vec<i32>) -> i32 {

    let mut nums = series.clone();

    // Example sequence find:
    // 1   3   6  10  15  21
    //   2   3   4   5   6
    //     1   1   1   1
    //       0   0   0
    // The 21 can be found by adding the last values before the num:
    // 0 + 1 + 5 + 15 = 21.
    // Next number would then be 0 + 1 + 6 + 21, and so on.

    let mut diff_acc = *nums.last().clone().unwrap();

    while !is_all_zero(&nums) {
        nums = nums
            .windows(2)
            .map(|c| c[1] - c[0])
            .collect();

        diff_acc += nums.last().clone().unwrap()
    }

    diff_acc

}

fn get_prev_number(series: &Vec<i32>) -> i32 {

    let mut nums = series.clone();

    let init_val = *nums.first().clone().unwrap();
    let mut diff_acc = 0;

    while !is_all_zero(&nums) {
        nums = nums
            .windows(2)
            .map(|c| c[0] - c[1])
            .collect();

        diff_acc -= nums.first().clone().unwrap()
    }
    
    init_val - diff_acc

}

fn is_all_zero(v: &Vec<i32>) -> bool {
    v.iter().filter(|&&n| n == 0).count() == v.len()
}