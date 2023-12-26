use crate::utils;

fn check1(lines: &str) -> u32 {
    let mut result = 0;
    for line in lines.split('\n') {
        let mut values = line.split([':', '|']);
        let mut count = 0;
        let numbers_sx: Vec<u32> = values
                    .nth(1)
                    .unwrap()
                    .split(' ')
                    .filter_map(|x| x.parse::<u32>().ok())
                    .collect();
        let numbers_dx: Vec<u32> = values
                .nth(0)
                .unwrap()
                .split(' ')
                .filter_map(|x| x.parse::<u32>().ok())
                .collect();
        for num in numbers_dx {
            if numbers_sx.contains(&num) {
                count += 1;
            }
        }

        if count > 0 {
            result += 2u32.pow(count - 1);
        }
    }
    result
}

fn check2(lines: &str) -> u32 {
    let mut result = 0;
    let mut scratchcards = Vec::new();
    let mut cardNUmber = 1;
    for line in lines.split('\n') {
        let mut values = line.split([':', '|']);
        let mut count = 0;
        let numbers_sx: Vec<u32> = values
                    .nth(1)
                    .unwrap()
                    .split(' ')
                    .filter_map(|x| x.parse::<u32>().ok())
                    .collect();
        let numbers_dx: Vec<u32> = values
                .nth(0)
                .unwrap()
                .split(' ')
                .filter_map(|x| x.parse::<u32>().ok())
                .collect();

        for num in numbers_dx {
            if numbers_sx.contains(&num) {
                count += 1;
            }
        }

        if cardNUmber > scratchcards.len(){
            scratchcards.push(1);
        }
        else {

        } 

        if count > 0 {
            for index in cardNUmber..cardNUmber + count {
                if index > scratchcards.len(){
                    scratchcards.push(1);
                }
            }
        }
    }
    result
}

pub fn run() {
    let lines = utils::my_read_file("examples/text_day4.txt").unwrap();
    println!("Day4 - Part1: {}", check1(&lines));
    println!("Day4 - Part2: {}", check2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day4_test1() {
        let test = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let result = check1(test);
        assert_eq!(result, 13);
    }

    #[test]
    fn day4_test2() {
        let test = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let result = check2(test);
        assert_eq!(result, 30);
    }
}