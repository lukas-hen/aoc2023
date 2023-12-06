#![allow(dead_code)]
#![allow(unused)]

use std::error::Error;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;

fn main() -> Result<(), Box<dyn Error>> {

    //day_01::run()
    //day_02::part_1();
    //day_02::part_2();
    //day_03::part_1()?;
    //day_04::part_1();
    //day_05::part_1();
    //day_05::part_2();
    //day_06::part_1();
    day_06::part_2();

    Ok(())
}