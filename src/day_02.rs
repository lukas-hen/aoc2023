use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};

#[allow(dead_code)]

pub fn part_1() -> Result<(), Box<dyn Error>> {
    
    let filepath = "data/day_02/1_real.in".to_string();
    let file = File::open(&filepath)?;
    let reader = BufReader::new(file);

    let mut games_possible: Vec<bool> = vec![];

    for line in reader.lines() {

        let l = line?; 

        // Below yields collection like f.e ["3 blue", "4 red", "1 red", "2 green", "6 blue", "2 green"]
        // Game delimiter is not needed. Impossibility can only be proven if one color exceeds 
        // the possible num for that color.

        let flattened_bag_pickups: Vec<&str> = l
            // Cant skip n first chars as Game number can become 2 or even 3 digit.
            // Find : first, and skip the whitespace after.;
            .split(": ")
            .last()
            .unwrap()
            .split(";")
            .map(|s| s.trim())
            .flat_map(|s| s.split(","))
            .map(|s| s.trim())
            .collect();

        
        games_possible.push(is_possible(flattened_bag_pickups, 12, 13, 14).unwrap());
    }

    let sum_possible_indices: usize = games_possible
        .iter()
        .enumerate()
        .filter(|tup| *tup.1 == true) // is possible
        .map(|tup| tup.0 + 1) // get indices, starting from ONE
        .sum();

    println!("{:?}", sum_possible_indices);

    Ok(())

}

pub fn part_2() -> Result<(), Box<dyn Error>> {
    
    let filepath = "data/day_02/1_real.in".to_string();
    let file = File::open(&filepath)?;
    let reader = BufReader::new(file);

    let mut max_products: Vec<i32> = vec![];

    for line in reader.lines() {

        let l = line?; 

        // Below yields collection like f.e ["3 blue", "4 red", "1 red", "2 green", "6 blue", "2 green"]
        // Game delimiter is not needed. Impossibility can only be proven if one color exceeds 
        // the possible num for that color.

        let flattened_bag_pickups: Vec<&str> = l
            // Cant skip n first chars as Game number can become 2 or even 3 digit.
            // Find : first, and skip the whitespace after.;
            .split(": ")
            .last()
            .unwrap()
            .split(";")
            .map(|s| s.trim())
            .flat_map(|s| s.split(","))
            .map(|s| s.trim())
            .collect();

            let (max_red, max_green, max_blue) = get_maxes(flattened_bag_pickups)?;


        max_products.push(max_red * max_green * max_blue);
    }

    let max_products_sum: i32 = max_products.iter().sum();

    println!("{}", max_products_sum);

    Ok(())

}


fn is_possible(takes_vec: Vec<&str>, max_possible_red: i32, max_possible_green: i32, max_possible_blue: i32) -> Result<bool, Box<dyn Error>> {
    
    let (max_red, max_green, max_blue) = get_maxes(takes_vec)?;

    Ok(max_red <= max_possible_red && max_green <= max_possible_green && max_blue <= max_possible_blue)
}


fn get_maxes(takes_vec: Vec<&str>) -> Result<(i32, i32, i32), Box<dyn Error>> {
    
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;

    for take in takes_vec {

        let split: Vec<&str> = take.split(" ").collect();

        match split[..] {
            [num, "red"] if str::parse::<i32>(num)? > max_red => max_red = str::parse::<i32>(num)?,
            [num, "green"] if str::parse::<i32>(num)? > max_green => max_green = str::parse::<i32>(num)?,
            [num, "blue"] if str::parse::<i32>(num)? > max_blue => max_blue = str::parse::<i32>(num)?,
            [] | [_, ..] => (),
        };

    }

    Ok((max_red, max_green, max_blue))
}