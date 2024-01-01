fn part1(input: &str) -> u32
{
    0
}


fn part2(input: &str) -> u32
{
    0
}

fn main() {
    let input = include_str!("./input.txt");
    println!("DayX - Part1: {}", part1(&input));
    println!("DayX - Part2: {}", part2(&input));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(0, part1(""));
    }

    #[test]
    fn test2() {
        assert_eq!(0, part2(""));
    }
}