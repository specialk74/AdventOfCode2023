use std::fs::read_to_string;

mod day1;
pub use crate::day1::{check1_day1, check2_day1};
mod day2;
pub use crate::day2::{check1_day2, check2_day2};
mod day3;
pub use crate::day3::{check1_day3, check2_day3};
mod day4;
pub use crate::day4::{check1_day4, check2_day4};

fn my_read_file(filename: &str) -> Result<String, ()> {
    if let Ok(lines) = read_to_string(filename) {
        return Ok(lines);
    }
    
    println!("{} Not Founded", filename);
    return Err(());
}

fn main() {

    // region: --- Day 1
    let lines = my_read_file("examples/text_day1.txt").unwrap();
    println!("Day1 - Part1: {}", check1_day1(&lines));
 
    let lines = my_read_file("examples/text_day1.txt").unwrap();
    println!("Day1 - Part2: {}", check2_day1(&lines));

    // endregion: --- Day 1

    // region: --- Day 2

    let lines = my_read_file("examples/text_day2.txt").unwrap();
    println!("Day2 - Part1: {}", check1_day2(&lines));

    let lines = my_read_file("examples/text_day2.txt").unwrap();
    println!("Day2 - Part2: {}", check2_day2(&lines));

    // endregion: --- Day 2

    // region: --- Day 3


    let lines = my_read_file("examples/text_day3.txt").unwrap();
    println!("Day3 - Part1: {}", check1_day3(&lines));

    let lines = my_read_file("examples/text_day3.txt").unwrap();
    println!("Day3 - Part2: {}", check2_day3(&lines));

    // endregion: --- Day 3

    // region: --- Day 4

    let lines = my_read_file("examples/text_day4.txt").unwrap();
    println!("Day4 - Part1: {}", check1_day4(&lines));

    let lines = my_read_file("examples/text_day4.txt").unwrap();
    println!("Day4 - Part2: {}", check2_day4(&lines));

    // endregion: --- Day 4
}
