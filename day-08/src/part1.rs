use std::collections::BTreeMap;
use nom::bytes::complete::tag;
use nom::sequence::preceded;
use nom::sequence::separated_pair;
use nom::character::complete::alpha0;
use nom::IResult;
use nom::Parser;
use nom::sequence::terminated;

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

pub fn process(input: &str) -> u32
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

    let mut count: u32 = 0;
    let mut it = navigate.chars().cycle();

    let mut last_father = "AAA";
    loop {
        count += 1;
        if let Some(c) = it.next() {

            if let Some((son1, son2)) = network.get(last_father) {
                if c == 'L' {
                    last_father = son1;
                }
                else {
                    last_father = son2;
                }
            }

            if last_father == "ZZZ" {
                break;
            }
        }   
    }

    count
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;

    #[rstest]
    #[case(
        "RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)",
        2
    )]
    #[case(
        "LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)",
        6
    )]
    fn test_process(
        #[case] input: &str,
        #[case] expected: u32,
    ) {
        assert_eq!(expected, process(input));
    }
}