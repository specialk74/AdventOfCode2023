#[derive(Debug, Clone)]
struct NumbersDay3 {
    number: u32,
    row: u32,
    start_col: u32,
    stop_col: u32,
}

impl NumbersDay3 {
    fn new(number: u32, row: u32, start_col: u32, stop_col: u32) -> Self {
        Self {
            number,
            row,
            start_col,
            stop_col
        }
    }
}

#[derive(Debug, Clone)]
struct SymbolsDay3 {
    row: u32,
    col: u32,
    car: char,
}

impl SymbolsDay3 {
    fn new(row: u32, col: u32, car: char) -> Self {
        Self {
            row,
            col,
            car
        }
    }
}

fn add_part1_day3_number(number_str: &mut String, numbers: &mut Vec<NumbersDay3>, row: u32, start_col: u32, col: u32) {
    if !number_str.is_empty() {
        numbers.push(NumbersDay3::new(number_str.parse::<u32>().unwrap(), row, start_col, col));
        number_str.clear();
    }
}

fn symbol_near_number(symbol: &SymbolsDay3, number: &NumbersDay3) -> bool {
    let limit_sx = if number.start_col > 0 {number.start_col - 1} else {number.start_col};
    let limit_dx = number.stop_col;

    if number.row > 0  && number.row - 1 == symbol.row {
        if symbol.col >= limit_sx && symbol.col <= limit_dx {
            return true;
        } 
    }

    if number.row == symbol.row {
        if limit_sx == symbol.col || limit_dx == symbol.col {
            return true;
        }
    }

    if number.row + 1 == symbol.row {
        if symbol.col >= limit_sx && symbol.col <= limit_dx {
            return true;
        } 

    }
    false
}

fn part1(input: &str) -> u32 {
    let mut result = 0;

    let mut row = 0;
    let mut numbers = Vec::new();
    let mut symbols = Vec::new();

    for line in input.split('\n') {
        let mut col = 0;
        let mut number_str = String::new();
        let mut start_col = 0;
        for car in line.chars() {
            match car {
                '.' => {
                    add_part1_day3_number(&mut number_str, &mut numbers, row, start_col, col);
                },
                '0'..='9' => {
                    if number_str.is_empty() {
                        start_col = col;
                    }
                    number_str.push(car);
                },
                ' ' => {},
                _ => {
                    add_part1_day3_number(&mut number_str, &mut numbers, row, start_col, col);
                    symbols.push(SymbolsDay3::new(row, col, car));
                },
            }
            col += 1;
        }
        add_part1_day3_number(&mut number_str, &mut numbers, row, start_col, col);
        row += 1;
    }

    for number in numbers {
        for symbol in symbols.clone() { 
            if symbol_near_number(&symbol, &number) {
                result += number.number;
                break;
            }
        }
    }
    
    result
}

fn get_ratio_day3(symbol: &SymbolsDay3, numbers: &Vec<NumbersDay3>) -> u32 {
    let mut first_number = 0;
    let mut first_number_founded = false;

    for number in numbers {
        if symbol_near_number(&symbol, &number) {
            if !first_number_founded {
                first_number = number.number;
                first_number_founded = true;
            }
            else {
                return first_number * number.number;
            }
        }
    }
    0
}

fn part2(input: &str) -> u32 {
    let mut result = 0;

    let mut row = 0;
    let mut numbers = Vec::new();
    let mut symbols = Vec::new();

    for line in input.split('\n') {
        let mut col = 0;
        let mut number_str = String::new();
        let mut start_col = 0;
        for car in line.chars() {
            match car {
                '.' => {
                    add_part1_day3_number(&mut number_str, &mut numbers, row, start_col, col);
                },
                '0'..='9' => {
                    if number_str.is_empty() {
                        start_col = col;
                    }
                    number_str.push(car);
                },
                ' ' => {},
                _ => {
                    add_part1_day3_number(&mut number_str, &mut numbers, row, start_col, col);
                    symbols.push(SymbolsDay3::new(row, col, car));
                },
            }
            col += 1;
        }
        add_part1_day3_number(&mut number_str, &mut numbers, row, start_col, col);
        row += 1;
    }

    for symbol in symbols{
        if symbol.car == '*' {
            result += get_ratio_day3(&symbol, &numbers);
        }
    }
    
    result
}

fn main() {
    let input = include_str!("./input.txt");
    println!("Day3 - Part1: {}", part1(&input));
    println!("Day3 - Part2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(4361, part1("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."));
    }

    #[test]
    fn test2() {
        assert_eq!(467835, part2("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."));
    }
}