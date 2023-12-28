use utils::my_read_file;

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
        for num in numbers_sx {
            if numbers_dx.contains(&num) {
                count += 1;
            }
        }

        if count > 0 {
            result += 2u32.pow(count - 1);
        }
    }
    result
}

// No: 10254412
// No: 12621882
// No: 18958970
// Yes: 12648035

fn check2(lines: &str) -> u32 {
    let mut result = 0;
    let mut scratchcards: Vec<u32> = Vec::new();
    let mut card_number = 0;
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

        // Check how many numbers present in the right list are present in the left list
        for num in numbers_sx {
            if numbers_dx.contains(&num) {
                count += 1;
            }
        }

        // If the Card Number is not present in the scratchcards list, then it is inserted with value 0
        if card_number >= scratchcards.len() {
            scratchcards.push(0);
        }

        // If it is present at least one number, start to create the childs
        if count > 0 {
            // Take the init value of the child
            let init_value = scratchcards[card_number] + 1;
            for index in card_number+1..=card_number + count {
                // If the child is not present, then it is created
                if index >= scratchcards.len() {
                    scratchcards.push(init_value);
                }
                // Otherwise it is updtaed
                else {
                    scratchcards[index] += init_value;
                }
            }
        }

        // At the end, the value is incremented of one value
        scratchcards[card_number] += 1;

        // Increment next card number value
        card_number += 1;
    }

    for num in scratchcards {
        result += num;
    }
    result
}

fn main() {
    let lines = my_read_file("examples/text_day4.txt").unwrap();
    println!("Day4 - Part1: {}", check1(&lines));
    println!("Day4 - Part2: {}", check2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
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
    fn test2() {
        let test = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
                        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
                        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
                        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
                        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
                        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let result = check2(test);
        assert_eq!(result, 30);
    }
    
    #[test]
    fn test3() {
        let test = "Card   1: 36 15 12 91 47 98 59 46 83 86 | 86 34 88  7 36 82 90 32 83 56 27 45 49 69 91 47 98 59 13 15 68 12 17 11 46
                        Card   2: 55 82  6 50 62  4 21 96 14 39 | 11 34 14 52  7 28 13 81 10 98 88 48 21 22 96  1 93 83 85 60 76 87 57 56 62
                        Card   3: 11 88 40  9  7 37 92 15 70 53 | 55 37 97 22 77 34 83 98  1 92 38  5 69 54 25 73 13 94 62 96 78 93 75 23 31
                        Card   4: 83 15  5  4  3 92 86  2 91 58 | 58  2 77 32 93 64 37 82 47 28 13 89 16 24 59 76 52 50 55 15 14 68 61 91 98
                        Card   5: 59 25  2 83 13 26 39 45 28 74 | 29 46 37 81 25 39  4  1  2 64 52 70 57 32 62 95 74  7 33 78 59 92 56  3 26
                        Card   6: 42 94 58 49 24 26 89 77 83 66 | 88  7 54 27 89 46 57 22 97 69 65 36 13 32 11 12 31 62 35 98  4 23 79 18 85
                        Card   7: 25  9 48 40 69 82 75 78 73 18 | 54 48 73 91 30 65 42 85 88 18 40 69 23 82 58 49 75 43 35 47 46 80 78  9 32";

        let result = check2(test);
        assert_eq!(result, 675);
    }

    #[test]
    fn test4() {
        let test = "Card   1: 36 15 12 91 47 98 59 46 83 86 | 86 34 88  7 36 82 90 32 83 56 27 45 49 69 91 47 98 59 13 15 68 12 17 11 46
                        Card   2: 55 82  6 50 62  4 21 96 14 39 | 11 34 14 52  7 28 13 81 10 98 88 48 21 22 96  1 93 83 85 60 76 87 57 56 62
                        Card   3: 11 88 40  9  7 37 92 15 70 53 | 55 37 97 22 77 34 83 98  1 92 38  5 69 54 25 73 13 94 62 96 78 93 75 23 31
                        Card   4: 83 15  5  4  3 92 86  2 91 58 | 58  2 77 32 93 64 37 82 47 28 13 89 16 24 59 76 52 50 55 15 14 68 61 91 98
                        Card   5: 59 25  2 83 13 26 39 45 28 74 | 29 46 37 81 25 39  4  1  2 64 52 70 57 32 62 95 74  7 33 78 59 92 56  3 26";

        let result = check2(test);
        assert_eq!(result, 159);
    }

    #[test]
    fn test5() {
        let test = "Card   1: 36 15 12 91 47 98 59 46 83 86 | 86 34 88  7 36 82 90 32 83 56 27 45 49 69 91 47 98 59 13 15 68 12 17 11 46
                        Card   2: 55 82  6 50 62  4 21 96 14 39 | 11 34 14 52  7 28 13 81 10 98 88 48 21 22 96  1 93 83 85 60 76 87 57 56 62
                        Card   3: 11 88 40  9  7 37 92 15 70 53 | 55 37 97 22 77 34 83 98  1 92 38  5 69 54 25 73 13 94 62 96 78 93 75 23 31
                        Card   4: 83 15  5  4  3 92 86  2 91 58 | 58  2 77 32 93 64 37 82 47 28 13 89 16 24 59 76 52 50 55 15 14 68 61 91 98";

        let result = check2(test);
        assert_eq!(result, 62);
    }

    #[test]
    fn test6() {
        let test = "Card   1: 36 15 12 91 47 98 59 46 83 86 | 86 34 88  7 36 82 90 32 83 56 27 45 49 69 91 47 98 59 13 15 68 12 17 11 46
                        Card   2: 55 82  6 50 62  4 21 96 14 39 | 11 34 14 52  7 28 13 81 10 98 88 48 21 22 96  1 93 83 85 60 76 87 57 56 62
                        Card   3: 11 88 40  9  7 37 92 15 70 53 | 55 37 97 22 77 34 83 98  1 92 38  5 69 54 25 73 13 94 62 96 78 93 75 23 31";

        let result = check2(test);
        assert_eq!(result, 29);
    }

    #[test]
    fn test7() {
        let test = "Card   1: 36 15 12 91 47 98 59 46 83 86 | 86 34 88  7 36 82 90 32 83 56 27 45 49 69 91 47 98 59 13 15 68 12 17 11 46
                        Card   2: 55 82  6 50 62  4 21 96 14 39 | 11 34 14 52  7 28 13 81 10 98 88 48 21 22 96  1 93 83 85 60 76 87 57 56 62";

        let result = check2(test);
        assert_eq!(result, 20);
    }

    #[test]
    fn test8() {
        let test = "Card   1: 36 15 12 91 47 98 59 46 83 86 | 86 34 88  7 36 82 90 32 83 56 27 45 49 69 91 47 98 59 13 15 68 12 17 11 46";

        let result = check2(test);
        assert_eq!(result, 11);
    }
}