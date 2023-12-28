use utils::my_read_file;

fn check1(input: &str) -> u32 {
    let mut result = 0;
    const MAX_BLUE: u32 = 14;
    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;

    for line in input.split('\n') {
        let game: Vec<&str> = line.split(':').collect();
        let gameid_str: Vec<&str>= game[0].split(' ').collect();
        //dbg!("{}", gameid_str.clone());
        let gameid = gameid_str[1].parse::<u32>().unwrap();
        let colors_str: Vec<&str> = game[1].split([',', ';']).collect();
        let mut add = true;
        for colors in colors_str {
            let color: Vec<&str> = colors.split_whitespace().collect();
            let number = color[0].parse::<u32>().unwrap();
            match color[1] {
                "green" => if number > MAX_GREEN { add = false },
                "red" => if number > MAX_RED { add = false },
                "blue" => if number > MAX_BLUE { add = false },
                _ => {},
            } 
                
        }

        if add {
            result += gameid;
        }
    }
    result
}

fn check2(input: &str) -> u32 {
    let mut result = 0;

    for line in input.split('\n') {
        let mut min_red: u32 = u32::MIN;
        let mut min_blue: u32 = u32::MIN;
        let mut min_green: u32 = u32::MIN;
        let game: Vec<&str> = line.split(':').collect();
        //dbg!("{}", gameid_str.clone());
        let colors_str: Vec<&str> = game[1].split([',', ';']).collect();
        for colors in colors_str {
            let color: Vec<&str> = colors.split_whitespace().collect();
            let number = color[0].parse::<u32>().unwrap();
            match color[1] {
                "green" => if number > min_green { min_green = number },
                "red" => if number > min_red { min_red = number },
                "blue" => if number > min_blue { min_blue = number },
                _ => {},
            } 
                
        }
        //dbg!("{} {} {} {}", min_green, min_red, min_blue, min_green * min_red * min_blue);
        result += min_green * min_red * min_blue;
    }
    result
}

fn main() {
    let lines = my_read_file("examples/text_day2.txt").unwrap();
    println!("Day2 - Part1: {}", check1(&lines));
    println!("Day2 - Part2: {}", check2(&lines));
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = check1(test);
        assert_eq!(result, 8);
    }
    
    #[test]
    fn test2() {
        let test = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = check2(test);
        assert_eq!(result, 2286);
    }
}