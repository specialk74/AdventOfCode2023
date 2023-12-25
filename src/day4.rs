pub fn check1_day4(lines: &str) -> u32 {
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

pub fn check2_day4(lines: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    // region: --- Day 1
    #[test]
    fn day4_test1() {
        let test = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let result = check1_day4(test);
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

        let result = check2_day4(test);
        assert_eq!(result, 30);
    }

    // endregion: --- Day 1
}