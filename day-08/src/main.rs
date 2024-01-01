use std::collections::BTreeMap;

use nom::bytes::complete::tag;
use nom::sequence::preceded;
use nom::sequence::separated_pair;
use nom::character::complete::alpha0;
use nom::IResult;
use nom::Parser;
use nom::sequence::terminated;

/* 
#[derive(Debug)]
struct Pippo {
    father: String,
    sons: (String,String)
}

impl Pippo {
    fn new(father: String, son1: String, son2: String) -> Self {
        Self {
            father,
            sons: (son1, son2)
        }
    }
}
*/
fn parse_sons(input: &str) -> IResult<&str, (&str, &str)> {    
    preceded(
        tag("("), 
        terminated(
            separated_pair(alpha0, tag(", "), alpha0), 
        tag(")"))
    )
    .parse(input)
}

fn parse_line(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    separated_pair(alpha0, tag(" = "), parse_sons).parse(input)
}

fn part1(input: &str) -> u32
{
    let mut iter = input.lines();
    let navigate = iter.next().unwrap(); // take only the first line
    iter.next().unwrap(); // jump the empty line
    
    let network: BTreeMap<String, (String, String)> = iter
        .map(|line| {
            let (_, (father, (son1, son2))) = parse_line(line.trim()).expect("Error parsing");
            //Pippo::new(father.to_string(), son1.to_string(), son2.to_string())
            (father.to_string(), (son1.to_string(), son2.to_string()))
        })
        .collect();

    todo!()
}


fn part2(input: &str) -> u32
{
    todo!()
}

fn main() {
    let input = include_str!("./input.txt");
    println!("Day08 - Part1: {}", part1(&input));
    println!("Day08 - Part2: {}", part2(&input));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(2, part1("RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)"));
    }

    #[test]
    fn test2() {
        assert_eq!(6, part1("LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)"));
    }
}