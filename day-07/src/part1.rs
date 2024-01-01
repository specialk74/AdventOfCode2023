use std::ops::Deref;
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
enum HandType{
    FiveOfAKind = 6, 
    FourOfAKind = 5,
    FullHouse = 4, 
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

fn score_hand(hand: &str) -> (HandType, (u32,u32,u32,u32,u32)){
    use self::HandType::*;

    // From "32T3K"
    let values = hand
        .chars() // Create an iterator
        .counts() // Counts the number of characters: create an HashMap with 'char' and 'u32'
        .values() // Create an iterator to check all element inside the previous HasMap
        .sorted() // Sorted the HaspMap
        .join(""); // Collect all elements in a String
    // To "1112"

    // From "1112"
    let hand_type = match values.deref() {
        "5" => FiveOfAKind,
        "14" => FourOfAKind,
        "23" => FullHouse,
        "113" =>  ThreeOfAKind,
        "122" => TwoPair,
        "1112" => OnePair,
        "11111" => HighCard,
        value => panic!("Invalid value {}", value)
    };
    // To HandType::OnePair

    // From 'A'
    let card_scores = hand.chars().map(|card| {
        match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        value => value.to_digit(10).unwrap()
    }})
    .collect_tuple()
    .unwrap();
    // To 14

    (hand_type, card_scores)
}

pub fn process(input: &str) -> u32
{
    input
        .lines() // For every line inside input
        .map(|line| {
            // From "32T3K 765"
            let (hand, bid) = line
                .trim()
                .split_once(" ")
                .unwrap();
            // To ("3273K", "765")

            // From "3273K 765"
            (
                hand, 
                bid.parse::<u32>().unwrap(), // From "765" to 765: convert string to number
                score_hand(hand) // From "3273K" to the tuple (HandType::OnePair, 3)
            )
            // Return a tuple: ("3273K", 765, (HandType::OnePair, (3,2,10,3,13)))
        })
        // Sort all values using number of the tuple
        .sorted_by_key(|x| (x.2 .0 as u8, x.2 .1)) // x.2 refers to the tuple (HandType, u32) while .1 refers to (u32,u32,u32,u32,u32) of the tuple that means the first char.
        .enumerate() // Transform the HasMap into an interetor with 0, <First Element of HashMap>; 1, <Second Element of HashMap>; and so on
        .map(|(index, (_hand, bid, _))| {
            (index as u32 + 1) * bid
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(6440, process("32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483"));
    }
}