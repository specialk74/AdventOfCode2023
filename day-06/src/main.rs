use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::character::complete::line_ending;
use nom::character::complete::space1;
use nom::IResult;
use nom::bytes::complete::is_not;
use nom::Parser;
use nom_supreme::parser_ext::ParserExt;
use nom::character::complete;

fn nums(input: &str) -> IResult<&str, Vec<u32>> {
    is_not("1234567890")
        .precedes(separated_list1(space1, complete::u32))
        .parse(input)
}

fn parse_times(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    separated_pair(nums, line_ending, nums).parse(input)
}
 
fn part1(input: &str) -> u32 {
    let (_, (times, distances)) = parse_times(input).expect("Error parsing");
    dbg!(times, distances);
    0
}

fn part2(input: &str) -> u64 {
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
        let input = "Time:      7  15   30
                     Distance:  9  40  200";
        assert_eq!(288, part1(input));
    }

    #[test]
    fn test2() {
        let result = part2("");
        assert_eq!(result, 0);
    }

    fn my_is_not_test_1() {
        dbg!(is_not::<_, _, Error<_>>("1234567890").parse("Time: 7 15 30"));
        assert!(false);
    }

    fn my_is_not_test_2() {
        dbg!(is_not::<_, _, Error<_>>("1234567890").precedes(separated_list1(space1, complete::u32)).parse("Time: 7 15 30"));
        assert!(false);
    }

    fn my_is_not_test_3() {
        dbg!(tag::<_, _, Error<_>>("Time: ").precedes(separated_list1(space1, complete::u32)).parse("Time: 7 15 30"));
        assert!(false);
    }
}