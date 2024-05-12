///
/// Day Seven Problem: Given a collection of symbols, assign values and sort
/// 
/// Solution: This solution maps the values to get the number of times a symbol (card)
/// appears and also assign a weight to each symbol. Then the collection of cards is 
/// sorted by the number of instances first, and then the weight of cards. Each bid is
/// multiplied by the index plus one. There is room for optimization with a great deal
/// of code duplication in this program which will be addressed later.
/// 
pub mod day_seven {
    use std::{cmp::Ordering, collections::HashMap};
    #[derive(Debug)]

    struct Card {
        bid: usize,
        card: String,
        hand_type: usize,
    }

    impl Card {
        fn new_card(card_string: &str) -> Card {
            let card_parts: Vec<&str> = card_string.split(' ').collect();
            let mut card_map = HashMap::new();
            for i in card_parts[0].chars() {
                card_map.entry(i).and_modify(|count| *count += 1).or_insert(1);
            }
            let mut card_nums: Vec<usize> = card_map.into_values().collect();
            card_nums.sort_by(|a, b| a.cmp(b).reverse());
            let hand_type = match card_nums[0] {
                5 => {
                    7
                }
                4 => {
                    6
                }
                3 => {
                    if card_nums[1] == 2 { 5 } else { 4 }
                }
                2 => {
                    if card_nums[1] == 2 { 3 } else { 2 }
                }
                1 => {
                    1
                }
                _ => {
                    0
                }
            };

            Card {bid: card_parts[1].parse::<usize>().unwrap_or(0), card: String::from(card_parts[0]), hand_type}
        }
        
        fn new_card_jokers(card_string: &str) -> Card {
            let card_parts: Vec<&str> = card_string.split(' ').collect();
            let mut card_map = HashMap::new();
            for i in card_parts[0].chars() {
                card_map.entry(i).and_modify(|count| *count += 1).or_insert(1);
            }
            let jokers = card_map.remove(&'J').unwrap_or(0);
            let mut card_nums: Vec<usize> = card_map.into_values().collect();
            card_nums.sort_by(|a, b| a.cmp(b).reverse());
            if card_nums.len() == 0 && jokers == 5 {
                card_nums = vec![5];
            } else {
                card_nums[0] += jokers;
            }
            let hand_type = match card_nums[0] {
                5 => {
                    7
                }
                4 => {
                    6
                }
                3 => {
                    if card_nums[1] == 2 { 5 } else { 4 }
                }
                2 => {
                    if card_nums[1] == 2 { 3 } else { 2 }
                }
                1 => {
                    1
                }
                _ => {
                    0
                }
            };
            Card {bid: card_parts[1].parse::<usize>().unwrap_or(0), card: String::from(card_parts[0]), hand_type}
        }

        fn compare(&self, card: &Card, jokers: bool) -> Ordering {
            let char_nums: HashMap<char, usize> = HashMap::from([('A', 14), ('K', 13), ('Q', 12), ('J', if jokers {1} else {11}), ('T', 10),]);
            if self.hand_type == card.hand_type {
                for i in 0..5 {
                    let card_char = self.card.chars().nth(i).unwrap_or('0');
                    let comp_char = card.card.chars().nth(i).unwrap_or('0');
                    let card_char = if card_char.is_numeric() { String::from(card_char).parse::<usize>().unwrap_or(0) } else { *char_nums.get(&card_char).unwrap_or(&0) };
                    let comp_char = if comp_char.is_numeric() { String::from(comp_char).parse::<usize>().unwrap_or(0) } else { *char_nums.get(&comp_char).unwrap_or(&0) };
                    if comp_char == card_char {
                        continue;
                    }
                    if card_char > comp_char { return Ordering::Greater; } else { return Ordering::Less; }
                }
            }
            if self.hand_type > card.hand_type { return Ordering::Greater; } else { return Ordering::Less; }
        }
    }

    fn part_one(string: &str) -> usize {
        let card_strings: Vec<&str> = string.split('\n').collect();
        let mut cards: Vec<Card> = Vec::new();
        let mut sum = 0;
        let mut index = 1;
        for i in card_strings {
            cards.push(Card::new_card(i));
        }
        cards.sort_by(|a, b| a.compare(b, false));
        for i in cards {
            sum += index * i.bid;
            index += 1;
        }
        sum
    }

    fn part_two(string: &str) -> usize {
        let card_strings: Vec<&str> = string.split('\n').collect();
        let mut cards: Vec<Card> = Vec::new();
        let mut sum = 0;
        let mut index = 1;
        for i in card_strings {
            cards.push(Card::new_card_jokers(i));
        }
        cards.sort_by(|a, b| a.compare(b, true));
        for i in cards {
            sum += index * i.bid;
            index += 1;
        }
        sum
    }

    pub fn test(string: &str) -> (usize, usize) {
        (part_one(string), part_two(string))
    }
}