use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead, Lines};

#[allow(dead_code)]
pub fn part_1() -> Result<(), Box<dyn Error>> {

    let filepath = "data/day_05/1_real.in".to_string();
    let file = File::open(&filepath)?;
    let reader = BufReader::new(file);

    let mut reader_it: Lines<BufReader<File>> = reader.lines();

    let first_line = &reader_it.next().unwrap()?;
    let input_seeds: Vec<i64> = parse_nums(&remove_colon_prefix(first_line));
    let mappings = get_all_mappings(reader_it);

    let mut mapped: Vec<i64> = vec![];
    let mut seed_state: i64;
    
    for seed in &input_seeds {
        seed_state = *seed;
        for mapping in &mappings {
            seed_state = apply_mapping(&seed_state, mapping)
        }

        mapped.push(seed_state);
    }

    println!("{:?}", mapped.iter().min().unwrap());

    Ok(())

}

#[allow(dead_code)]
pub fn part_2() -> Result<(), Box<dyn Error>> {

    let filepath = "data/day_05/1_real.in".to_string();
    let file = File::open(&filepath)?;
    let reader = BufReader::new(file);

    let mut reader_it: Lines<BufReader<File>> = reader.lines();

    let first_line = &reader_it.next().unwrap()?;
    let input_seeds: Vec<i64> = parse_nums(&remove_colon_prefix(first_line));
    let seeds_as_ranges = parse_seeds_as_ranges(&input_seeds);
    let mappings = get_all_mappings(reader_it);

    let mut mapped: Vec<i64> = vec![];
    let mut seed_state: i64;
    let mut count: i64 = 0;

    for seed in &seeds_as_ranges {
        seed_state = *seed;
        for mapping in &mappings {
            seed_state = apply_mapping(&seed_state, mapping)
        }
        mapped.push(seed_state);

        count += 1;
        if count % 10000000 == 0 {
            println!("Finished {} seeds out of {}", count, seeds_as_ranges.len());
        }
    }

    println!("{:?}", mapped.iter().min().unwrap());

    Ok(())

}

fn parse_seeds_as_ranges(seeds: &Vec<i64>) -> Vec<i64> {
    let mut out: Vec<i64> = vec![];
    for pair in seeds.chunks(2) {
        let [start, range] = *pair else { panic!("Seed list not even!") };
        for n in start..start+range {
            out.push(n);
        }
    }
    out
}

fn apply_mapping(input: &i64, mapping: &Vec<(i64, i64, i64)>) -> i64 {

    let mapping_for_input = mapping.iter()
        // Where input is between lower & upper bound.
        .find(|t| input >= &t.0 && input < &t.1);

    match mapping_for_input {
        Some(v) => input.clone() + v.2,
        None => input.clone(),
    }   
}

fn get_all_mappings(it: Lines<BufReader<File>>) -> Vec<Vec<(i64, i64, i64)>> {

    let mut mappings: Vec<Vec<(i64, i64, i64)>> = vec![];
    let mut temp_vec: Vec<String> = vec![];

    for line in it {
        let temp_line = line.unwrap();

        if temp_line == "" {
            continue
        }

        if is_mapping_header(&temp_line) && temp_vec.len() > 0 {
            mappings.push(parse_mapping(&temp_vec));
            temp_vec = vec![];

        } else if is_mapping_header(&temp_line) && temp_vec.len() == 0 {
            // First line, dont push to result vec.
            continue;
        } else {
            temp_vec.push(temp_line);
        }
    }

    // Last range doesn't end with a mapping header.
    if temp_vec.len() > 0 {
        mappings.push(parse_mapping(&temp_vec));
        temp_vec = vec![];
    }

    mappings

}

fn parse_mapping(s_vec: &Vec<String>) -> Vec<(i64, i64, i64)> {

    // To avoid looping over huge ranges, a mapping consists of:
    // (lower_bound, upper_bound, diff)
    // if val is between low/upper bound - diff should be added.

    let mut mappings: Vec<(i64, i64, i64)> = vec![];

    for line in s_vec {
        //dst src range
        let nums = parse_nums(line);
        let [dst, src, range] = *nums.as_slice() else {todo!()};
        
        mappings.push((src, src + range, dst - src));
        
    }

    mappings
}   

fn remove_colon_prefix(s: &String) -> String {
    s.split(":").last().unwrap().trim().to_string()
}

fn parse_nums(num_str: &String) -> Vec<i64> {
    num_str
        .split_whitespace()
        .map(|ns| ns.parse::<i64>().unwrap())
        .collect()
}

fn is_mapping_header(s: &String) -> bool {
    s.contains("map:")
}
