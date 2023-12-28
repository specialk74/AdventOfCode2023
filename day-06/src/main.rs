fn part1(_lines: &str) -> u64 {
    let result = 0;
    for _line in _lines.split("\n") {

    }
    result
}

fn part2(_lines: &str) -> u64 {
    let result = 0;
    result
}

fn main() {
    let input = include_str!("./input.txt");
    println!("Day6 - Part1: {}", part1(&input));
    println!("Day6 - Part2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = part1("Time:      7  15   30
        Distance:  9  40  200");
        assert_eq!(result, 288);
    }

    #[test]
    fn test2() {
        let result = part2("");
        assert_eq!(result, 0);
    }
}