use std::fmt::Debug;
use std::error::Error;
use std::fs::File;
use std::io::{ BufReader, BufRead };

use num::Integer;



pub fn part_1() -> Result<(), Box<dyn Error>> {

    let file_path = "data/day_10/1_real.in".to_string();
    let file = File::open(file_path)?;
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();

    let grid = Grid::from_lines(&lines);
    let starting_position = grid.find('S').unwrap();

    let mut cur = GridCursor::new(&grid, starting_position.0, starting_position.1);

    println!("start_val: {}", cur.value().unwrap());
    println!("start_pos: {}, {}", starting_position.0, starting_position.1 );

    let start_neighbors =  vec![
        (Direction::North, cur.peek(Direction::North)),
        (Direction::South, cur.peek(Direction::South)),
        (Direction::West, cur.peek(Direction::West)),
        (Direction::East, cur.peek(Direction::East)),
    ];

    //println!("{:?}", start_neighbors);

    // let valid_starts = [
    //         (Direction::East, Some(&'7')),
    //         (Direction::North, Some(&'7')),
    //         (Direction::North, Some(&'|')),
    //         (Direction::South, Some(&'|')),
    //         (Direction::South, Some(&'L')),
    //         (Direction::West, Some(&'L')),
    //         (Direction::East, Some(&'J')),
    //         (Direction::South, Some(&'J')),
    //         (Direction::East, Some(&'-')),
    //         (Direction::West, Some(&'-')),
    //     ];

    // let any_start_dir: Direction = start_neighbors
    //     .iter()
    //     .filter(|&n| n.1.is_some())
    //     .filter(|&n| valid_starts.contains(n))
    //     .next() // Doesn't matter what dir to start in.
    //     .unwrap()
    //     .0
    //     .clone(); 
    
    let loop_len = traverse_entire_loop(&mut cur, Direction::East);
    println!("{:?}", loop_len/2);

    Ok(())
}

fn traverse_entire_loop(cur: &mut GridCursor, start_dir: Direction) -> u32 {
    
    let mut cur_val = cur.traverse(start_dir).value().unwrap().clone();
    let mut n_traversals: u32 = 1;

    while cur_val != 'S' { 

        println!("{}", cur_val);

        match cur_val {
            '7' if cur.last_transition == Direction::North => cur.west(),
            '7' if cur.last_transition == Direction::East => cur.south(),
            '|' if cur.last_transition == Direction::North => cur.north(),
            '|' if cur.last_transition == Direction::South => cur.south(),
            'L' if cur.last_transition == Direction::South => cur.east(),
            'L' if cur.last_transition == Direction::West => cur.north(),
            'J' if cur.last_transition == Direction::East => cur.north(),
            'J' if cur.last_transition == Direction::South => cur.west(),
            '-' if cur.last_transition == Direction::East => cur.east(),
            '-' if cur.last_transition == Direction::West => cur.west(),
            'F' if cur.last_transition == Direction::North => cur.east(),
            'F' if cur.last_transition == Direction::West => cur.south(),
            s => { println!("err: {}", s); panic!("err") },
        };

        cur_val = *cur.value().unwrap();
        n_traversals += 1;

    }

    n_traversals    
}


#[derive(Debug)]
struct Grid {
    data: Vec<char>,
    n_rows: usize,
    n_cols: usize,
}

impl Grid {

    fn from_lines(lines: &Vec<String>) -> Self {
        // Assuming line width is equal without error checking.
        let n_rows = lines.len();
        let n_cols = lines[0].len();

        let data: Vec<char> = lines
            .iter()
            .flat_map(|line| line.chars())
            .collect();

        Grid { data, n_rows, n_cols }
    }

    fn at(&self, row_num: usize, col_num: usize) -> Option<&char> {

        if row_num >= self.n_rows || col_num >= self.n_cols {
            return None
        }

        Some(&self.data[row_num*self.n_cols + col_num])
    }

    fn find(&self, c: char) -> Option<(usize, usize)> {
        // Can only find first occurence

        let pos = self.data.iter().position(|&ele| ele == c);
        
        match pos {
            None => None,
            Some(n) => Some((n.div_floor(&self.n_rows), n % self.n_cols))
        }
    }
}

#[derive(Debug)]
struct GridCursor<'g> {
    x: i32, // Need to allow out of bounds (idx < 0) even if result will be none.  
    y: i32, // Thus doing signed ints here.
    grid: &'g Grid,
    last_transition: Direction,
}

impl<'g> GridCursor<'g> {

    fn new(grid: &'g Grid, x: usize, y: usize) -> Self {
        GridCursor{
            x: y as i32, y: x as i32, grid, last_transition: Direction::NoDirection
        }
    }

    fn value(&self) -> Option<&char> {
        if self.x < 0 || self.y < 0 {
            None
        } else {
            self.grid.at(self.y as usize, self.x as usize)
        }
    }

    fn traverse(&mut self, d: Direction) -> &mut Self {

        match d {
            Direction::North => self.north(),
            Direction::South => self.south(),
            Direction::West => self.west(),
            Direction::East => self.east(),
            Direction::NoDirection => self,
        }

    }

    fn north(&mut self) -> &mut Self {
        self.y -= 1;
        self.last_transition = Direction::North;
        self
    }

    fn south(&mut self) -> &mut Self {
        self.y += 1;
        self.last_transition = Direction::South;
        self
    }

    fn west(&mut self) -> &mut Self {
        self.x -= 1;
        self.last_transition = Direction::West;
        self
    }

    fn east(&mut self) -> &mut Self {
        self.x += 1;
        self.last_transition = Direction::East;
        self
    }

    fn peek(&self, direction: Direction) -> Option<&char> {
        
        match direction {
            Direction::North => self._bound_checked_at(self.x, self.y - 1),
            Direction::South => self._bound_checked_at(self.x, self.y + 1),
            Direction::West => self._bound_checked_at(self.x - 1, self.y),
            Direction::East => self._bound_checked_at(self.x + 1, self.y),
            Direction::NoDirection => self.value(),
        }

    }

    fn _bound_checked_at(&self, x: i32, y: i32) -> Option<&char> {
        
        if self.x < 0 || self.y < 0 {
            None
        } else {
            self.grid.at(y as usize, x as usize)
        }
        
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    West,
    East,
    NoDirection,
}