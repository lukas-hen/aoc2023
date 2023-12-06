use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn part_1() -> Result<(), Box<dyn Error>>{

    let filepath = "data/day_03/1_real.in".to_string();
    let file = File::open(&filepath)?;
    let reader = BufReader::new(file);

    let mut data: Vec<Vec<char>> = vec![];

    for line in reader.lines() {
        data.push(line?.chars().collect());
    }

    let n_rows = data.len();
    let n_cols = data[0].len();
    let data_flat: Vec<char> = data.iter().flat_map(|c| c.clone()).collect();

    let arr_2d: SafeCharArr2d = SafeCharArr2d { data: data_flat, n_cols, n_rows};
    
    let mut nums_w_adjacent_symbol: Vec<String> = vec![];

    for r in 0..n_rows {
        let mut c = 0;

        // start iterate over row
        while c < n_cols {

            // if number found, iterate over num.
            if data[r][c].is_ascii_digit() {
                
                let dig_len = scan_number_len(&data[r][c..]);
                let mut has_surrounding_symbol = false;
                let mut digit_str = String::new();

                for dig_idx in 0..dig_len {
                    
                    if arr_2d.has_symbol_neighbour(r, c+dig_idx) {
                        has_surrounding_symbol = true
                    }

                    digit_str.push(arr_2d.get(r, c+dig_idx).clone());
                
                }

                if has_surrounding_symbol {
                    nums_w_adjacent_symbol.push(digit_str);
                }

                // skip num
                c += dig_len;
                
            } else {
                c += 1;  
            }

       
        
        }
    }

    let sum: i32 = nums_w_adjacent_symbol.iter()
        .map(|ds| ds.parse::<i32>().unwrap())
        .sum();

    println!("{:?}", sum);

    Ok(())
}


#[allow(unused)]
pub fn part_2() -> Result<(), Box<dyn Error>>{

    let filepath = "data/day_03/1_tst.in".to_string();
    let file = File::open(&filepath)?;
    let reader = BufReader::new(file);

    let mut data: Vec<Vec<char>> = vec![];

    for line in reader.lines() {
        data.push(line?.chars().collect());
    }

    let n_rows = data.len();
    let n_cols = data[0].len();
    let data_flat: Vec<char> = data.iter().flat_map(|c| c.clone()).collect();

    let arr_2d: SafeCharArr2d = SafeCharArr2d { data: data_flat, n_cols, n_rows};

    for r in 0..n_rows {
        let mut c = 0;

        // start iterate over row
        while c < n_cols {

            // if number found, iterate over num.
            if data[r][c] == '*' {
                arr_2d.get_neighbours(r, c);
            }   
        }
    }

    Ok(())
}


fn scan_number_len(slice: &[char]) -> usize {

    // Takes slice and position for a digit and 
    // finds digits to the left and right to form the full number.

    if !&slice[0].is_ascii_digit() {
        return 0;
    }

    let mut num_len: usize = 0;

    for c in &slice[0..] {
        if c.is_ascii_digit() {
            num_len += 1;
        } else {
            break;
        }
    }

    num_len
    
}


fn scan_number(slice: &[char]) -> Option<String> {

    // Takes slice and position for a digit and 
    // finds digits to the left and right to form the full number.

    if !&slice[0].is_ascii_digit() {
        return None;
    }

    let mut nums: String = String::new();

    for c in &slice[0..] {
        if c.is_ascii_digit() {
            nums.push(c.clone());
        } else {
            break;
        }
    }

    Some(nums)
    
}

fn is_symbol(s: &char) -> bool {
    match s {
        '.' => false,
        _ if s.is_ascii_digit() => false,
        _ => true,
    }
}

struct SafeCharArr2d {
    data: Vec<char>,
    n_cols: usize,
    n_rows: usize,
}

impl SafeCharArr2d {

    fn get(&self, r: usize, c: usize ) ->  &char {
        self.data.get(r * self.n_rows + c).unwrap()
    }

    fn signed_get(&self, r: isize, c: isize) ->  Option<&char> {

        if r < 0 || c < 0 {
            return None
        }

        // lt 0 guarded r & c
        let r_g = r as usize;
        let c_g = c as usize;

        if r_g >= self.n_rows {
            None
        } else if c_g >= self.n_cols {
            None
        } else {
            Some(&self.data[r_g * self.n_rows + c_g])
        }
    }

    fn get_neighbours(&self, r: usize, c: usize) -> Vec<&char> {

        let mut n_vec: Vec<Option<&char>> = vec![];

        // usize is unsigned and fwiw the only acceptable index for a vec.
        // It is also the index type when using n..m looping.
        // since we need to be able to represent possible oob cases (they should return None)
        // we cast size to signed, pass a safe .get implementation (returns Option)
        // which checks bounds & then casts to usize again.
        
        // The reason we cannot just use safe sub/add for usize as overflow guard
        // is that if overflow occurs it returns usize min (0) or usize max which
        // might lead to double counting valid indexes.

        let r_u = r as isize;
        let c_u = c as isize;

        let top_idx: (isize, isize) = (r_u - 1, c_u);
        let bot_idx: (isize, isize) = (r_u + 1, c_u);
        let left_idx: (isize, isize) = (r_u, c_u - 1);
        let right_idx: (isize, isize) = (r_u, c_u + 1);
        let top_l_idx: (isize, isize) = (r_u - 1, c_u - 1);
        let top_r_idx: (isize, isize) = (r_u - 1, c_u + 1);
        let bot_l_idx: (isize, isize) = (r_u + 1, c_u - 1);
        let bot_r_idx: (isize, isize) = (r_u + 1, c_u + 1);

        n_vec.push(self.signed_get(top_l_idx.0, top_l_idx.1));
        n_vec.push(self.signed_get(top_idx.0, top_idx.1));
        n_vec.push(self.signed_get(top_r_idx.0 , top_r_idx.1));
        n_vec.push(self.signed_get(right_idx.0, right_idx.1));
        n_vec.push(self.signed_get(bot_r_idx.0, bot_r_idx.1));
        n_vec.push(self.signed_get(bot_idx.0, bot_idx.1));
        n_vec.push(self.signed_get(bot_l_idx.0, bot_l_idx.1));
        n_vec.push(self.signed_get(left_idx.0, left_idx.1));

        let neighbours: Vec<&char> = n_vec.iter()
            .filter(|n| n.is_some())
            .map(|opt| opt.clone().unwrap())
            .collect();
        
        neighbours

    }

    
    fn get_unique_neighbour_nums(&self, r: usize, c: usize) {

        let neighbours = self.get_neighbours(r, c);

        let unique_nums: Vec<String> = vec![];

        // neighbours start at top left & every index moves clockwise around center.
        let top_vec = vec![neighbours[0].clone(), neighbours[1].clone(), neighbours[2].clone()];
        let right = vec![neighbours[3].clone()];
        let bot_vec = vec![neighbours[6].clone(), neighbours[5].clone(), neighbours[4].clone()];
        let left = vec![neighbours[7].clone()];

        let mut last_was_digit = false;
        let mut digit_str = String::new();

        for i in 0..top_vec.len() {
            let num_len = scan_number_len(&top_vec[i..]);
            
            if num_len == 3 { 

            }
        }
    }

    fn has_symbol_neighbour(&self, r: usize, c: usize) -> bool {
        self.get_neighbours(r, c).iter()
            .filter(|c| is_symbol(c))
            .count() > 0
    }

    fn has_digit_neighbour(&self, r: usize, c: usize) -> bool {
        self.get_neighbours(r, c).iter()
            .filter(|c| c.is_ascii_digit())
            .count() > 0
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_symbol() {
        assert!(is_symbol(&'5') == false);
        assert!(is_symbol(&'.') == false);
        assert!(is_symbol(&'a') == true);
        assert!(is_symbol(&'*') == true); 
    }

    #[test]
    fn test_scan_num_len() {
        let v = vec!['a', '1', '2', '3' ,'c'];
        assert!(scan_number_len(&v[1..]) == 3);
        assert!(scan_number_len(&v[2..]) == 2);
        assert!(scan_number_len(&v[0..]) == 0);
    }
}