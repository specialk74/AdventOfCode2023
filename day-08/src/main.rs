mod part1;
mod part2;

fn main() {
    let input = include_str!("./input1.txt");
    println!("Day08 - Part1: {}", part1::process(&input));
    let input = include_str!("./input2.txt");
    println!("Day08 - Part2: {}", part2::process(&input));
}
