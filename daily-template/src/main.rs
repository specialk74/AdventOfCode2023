fn part1(lines: &str) -> u32
{
    0
}


fn part2(lines: &str) -> u32
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
        let result = part1("");
        assert_eq!(result, 0);
    }

    #[test]
    fn test2() {
        let result = part2("");
        assert_eq!(result, 0);
    }
}