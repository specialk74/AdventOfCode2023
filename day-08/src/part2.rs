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

    let mut count: u64 = 0;
    let mut it_cycle = navigate.chars().cycle();

    let mut last_fathers: Vec<&str> = network.iter().filter_map(|(father, _)| {
        if father.chars().last().unwrap() == 'A' {
            Some(*father)
        }
        else {
            None
        }
    })
    .collect();
    dbg!(&last_fathers);

    loop {
        count += 1;
        let c = it_cycle.next().unwrap();

        last_fathers = last_fathers
                        .iter()
                        .map(|father| {
                            let (son_left, son_right) = network.get(father).unwrap();
                            if c == 'L' {
                                *son_left
                            }
                            else {
                                *son_right
                            }
                        })
                        .collect();

        let fathers: Vec<&&str> = last_fathers.iter().filter(|last_father| {
            last_father.chars().last().unwrap() == 'Z'
        }).collect();
        
        if fathers.len() > 2
        {
            dbg!(&last_fathers);
        }

        if last_fathers.par_iter().all(|father| {
            father.chars().last().unwrap() == 'Z'
        }) {
            break;
        }

        if count % 1_000_000 == 0 {
            dbg!(count);
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