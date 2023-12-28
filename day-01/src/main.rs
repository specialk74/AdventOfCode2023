fn part1(input: &str) -> u32
{
    let mut sum = 0;
    for line in input.split('\n') {
        let t: String = line.chars().filter(|c| c.is_ascii_digit()).collect();
        if let Some(g) = t.chars().next()
        {
            sum += g.to_digit(10).unwrap() * 10;
        }
        if let Some(f)= t.chars().last() 
        {
            sum += f.to_digit(10).unwrap();
        }
    }
    sum
}

fn part1_iterator(input: &str) -> u32 {
    input
        .lines()
        .map(|line|{
            let mut it =
                line.chars().filter_map(|c| {
                    c.to_digit(10)
                });
            let first = it.next().expect("Should be a number");

            match it.last() {
                Some(num) => format!("{first}{num}"),
                None => format!("{first}{first}")
            }
            .parse::<u32>()
            .expect("Should be a valid number")
        })
        .sum::<u32>()
}


fn part2(input: &str) -> u32
{
    let vec_num = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let results = input.split('\n');
    let mut sum = 0;
    //let mut riga = 0;
    for line in results {
        let mut value_digit: u32 = 0;
        let mut digit_pos_founded = usize::MAX;
        if let Some(digit_pos) = line.find(|c: char| c.is_ascii_digit())
        {
            digit_pos_founded = digit_pos;
            if let Some(value) = line.chars().nth(digit_pos)
            {
                value_digit = value.to_digit(10).unwrap();
            }
        }

        if digit_pos_founded != 0
        {
            let mut occurence = 1;
            for number in &vec_num 
            {
                if let Some(index) = line.find(number)
                {
                    if index < digit_pos_founded
                    {
                        digit_pos_founded = index;
                        value_digit = occurence;
                    }
                }
                occurence += 1;
            }
        }

        //let first_number = value_digit;
        sum += value_digit * 10;


        let mut digit_pos_founded = 0;
        if let Some(digit_pos) = line.rfind(|c: char| c.is_ascii_digit())
        {
            digit_pos_founded = digit_pos;
            if let Some(value) = line.chars().nth(digit_pos)
            {
                value_digit = value.to_digit(10).unwrap();
            }
        }

        if digit_pos_founded != line.len() - 1
        {
            let mut occurence = 1;
            for number in &vec_num 
            {
                if let Some(index) = line.rfind(number)
                {
                    if index > digit_pos_founded
                    {
                        digit_pos_founded = index;
                        value_digit = occurence;
                    }
                }
                occurence += 1;
            }
        }
        sum += value_digit;

        //println!("Riga {} {} {}", riga, first_number, value_digit);
        //riga += 1;
    }
    sum
}

fn main() {
    let input = include_str!("./input.txt");
    println!("Day1 - Part1: {}", part1(&input));
    println!("Day1 - Part2: {}", part2(&input));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = part1("1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet");
        assert_eq!(result, 142);
    }

    #[test]
    fn test2() {
        let result = part2("two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen");
        assert_eq!(result, 281);
    }
}