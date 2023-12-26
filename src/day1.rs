use crate::utils;

fn check1(lines: &str) -> u32
{
    let results = lines.split('\n');
    let mut sum = 0;
    for line in results {
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


fn check2(lines: &str) -> u32
{
    let vec_num = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let results = lines.split('\n');
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

pub fn run() {
    let lines = utils::my_read_file("examples/text_day1.txt").unwrap();
    println!("Day1 - Part1: {}", check1(&lines));
     println!("Day1 - Part2: {}", check2(&lines));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_test1() {
        let test = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

        let result = check1(test);
        assert_eq!(result, 142);
    }

    #[test]
    fn day1_test2() {
        let test = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";

        let result = check2(test);
        assert_eq!(result, 281);
    }
}