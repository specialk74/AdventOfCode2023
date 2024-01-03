mod part1;
mod part2;

fn main() {
    let input = include_str!("./input.txt");
    println!("DayX - Part1: {}", part1::process(&input));
    println!("DayX - Part2: {}", part2::process(&input));
}
