use std::collections::BTreeMap;
use nom::bytes::complete::tag;
use nom::sequence::preceded;
use nom::sequence::separated_pair;
use nom::character::complete::alphanumeric0;
use nom::IResult;
use nom::Parser;
use nom::sequence::terminated;
use rayon::prelude::*; 

fn parse_sons(input: &str) -> IResult<&str, (&str, &str)> {    
    preceded(
        tag("("), 
        terminated(
            separated_pair(alphanumeric0, tag(", "), alphanumeric0), 
        tag(")"))
    )
    .parse(input)
}

fn parse_line(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    separated_pair(alphanumeric0, tag(" = "), parse_sons).parse(input)
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let tmp = a;
        a = b;
        b = tmp % b;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn search_single_path(network: &BTreeMap<&str, (&str, &str)>, navigate: &str, last_fathers_orig: &str) -> u64 {
    let mut count: u64 = 0;
    let mut it_cycle = navigate.chars().cycle();
    let mut last_fathers = last_fathers_orig;
    loop {
        count += 1;
        let c = it_cycle.next().unwrap();

        let (left, right) = network.get(last_fathers).unwrap();
        if c == 'L' {
            last_fathers = left;
        }
        else {
            last_fathers = right;
        }

        if last_fathers.ends_with("Z") {
            break;
        }
    }
    count
}

pub fn process(input: &str) -> u64
{
    let mut iter = input.lines();
    let navigate = iter.next().unwrap(); // take only the first line
    iter.next().unwrap(); // jump the empty line
    let network: BTreeMap<&str, (&str, &str)> = iter
        .map(|line| {
            let (_, (father, (son1, son2))) = parse_line(line.trim()).expect("Error parsing");
            (father, (son1, son2))
        })
        .collect();
/* 
    let last_fathers: Vec<&&str> = network
        .keys()
        .filter(|key| key.ends_with("A"))
        .collect();

    let cycles: Vec<u64> = last_fathers
    .iter()
    .map (|father| {
        search_single_path(&network, navigate, father)
    })
    .collect();

    cycles
        .into_iter()
        .fold(1, |acc, item| lcm(acc, item))
*/
    network
        .keys()
        .filter(|key| key.ends_with("A"))
        .collect::<Vec<&&str>>()
        .par_iter()
        .map (|father| {
            search_single_path(&network, navigate, father)
        })
        .collect::<Vec<u64>>()
        .into_iter()
        .fold(1, |acc, item| lcm(acc, item))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;

    #[rstest]
    #[case(
        "LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)",
        6
    )]    
    fn test_process(
        #[case] input: &str,
        #[case] expected: u64,
    ) {
        assert_eq!(expected, process(input));
    }
}