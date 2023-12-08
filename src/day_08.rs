use std::collections::HashMap;
use std::fmt::Debug;
use std::error::Error;
use std::fs::File;
use std::io::{ BufReader, BufRead };
use num::integer::lcm;


pub fn part_1() -> Result<(), Box<dyn Error>> {

    let file_name = "data/day_08/1_real.in".to_string();
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    let instructions = lines.next().unwrap()?;
    let _ = lines.next().unwrap()?; // Empty line separator.
    
    let mut nodes: HashMap<String, Directions> = HashMap::new();

    for line in lines {

        let binding = line?;
        let value = binding.split(" = ").next().unwrap();
        let directions_str = binding.split(" = ").last().unwrap();
        let directions = Directions::from_str(directions_str).unwrap();
        nodes.insert(value.to_string(), directions);

    };

    let num_runs = calc_traversal_len(&nodes, instructions, &String::from("AAA"), |s| s.ends_with("Z"));

    println!("{}", num_runs);

    Ok(())
}


pub fn part_2() -> Result<(), Box<dyn Error>> {

    let file_name = "data/day_08/1_real.in".to_string();
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    let instructions = lines.next().unwrap()?;
    let _ = lines.next().unwrap()?; // Empty line separator.
    
    let mut nodes: HashMap<String, Directions> = HashMap::new();

    for line in lines {

        let binding = line?;
        let value = binding.split(" = ").next().unwrap();
        let directions_str = binding.split(" = ").last().unwrap();
        let directions = Directions::from_str(directions_str).unwrap();
        nodes.insert(value.to_string(), directions);

    };

    let starting_states: Vec<String> = nodes
        .keys()
        .map(|s| s.clone())
        .filter(|s| s.ends_with("A"))
        .collect();
    
    let traversal_lengths: Vec<i64> = starting_states
        .iter()
        .map(|start| calc_traversal_len(&nodes, instructions.clone(), &start, |s| s.ends_with("Z")))
        .collect();

    let lcm = traversal_lengths.iter().fold(1, |acc, &n| lcm(acc, n));

    println!("{:?}", lcm);

    Ok(())
}


fn calc_traversal_len<F>(nodes: &HashMap<String, Directions>, instructions: String, from: &String, end_condition: F) -> i64 
where F: Fn(&String) -> bool
{

    let mut state = from.clone();
    let mut num_runs: i64 = 0;

    for instruction in instructions.chars().cycle() {

        let dirs = nodes.get(&state).unwrap();
        
        if instruction == 'L' {
            state = dirs.left.clone();
        } else if instruction == 'R' {
            state = dirs.right.clone();
        }

        num_runs += 1;

        if end_condition(&state) { break; }
    }

    num_runs
}

#[derive(Debug)]
struct Directions {
    left: String,
    right: String,
}

impl Directions {
    fn from_str(directions_str: &str) -> Result<Self, String> {

        let no_paren = directions_str
            .replace("(", "")
            .replace(")", "");

        let left_and_right: Vec<&str> = no_paren
            .split(", ")
            .collect();

        let left = left_and_right[0];
        let right = left_and_right[1];

        Ok(
            Directions{
                left: left.to_string(),
                right: right.to_string(),
            }
        )
    }
}
