///
/// Day One Problem: Derive numbers from strings by getting the first and last digit
/// 
/// Solution: Both functions use a two direction approach with the first checking one 
/// by one for a numeric digit and the second including the spellings of digits.
/// 

pub mod day_one {
    use std::collections::HashMap;

    fn number_from_string_first(str: &str) -> u32 {
        let mut chars = str.chars();
        let mut first: u32 = 0;
        let mut second: u32 = 0;
        loop {
            match chars.next() {
                Some(char) => {
                    if char.is_numeric() {
                        first = char.to_digit(10).unwrap();
                        second = first;
                        break;
                    }
                },
                None => {break;}
            }
        }
        let mut chars = chars.rev();
        loop {
            match chars.next() {
                Some(char) => {
                    if char.is_numeric() {
                        second = char.to_digit(10).unwrap();
                        break;
                    }
                },
                None => {break;}
            }
        }
        first * 10 + second
    }
    fn number_from_string_second(str: &str) -> u32 {
        let nums: HashMap<&str, u32> = HashMap::from([("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)]);
        let mut chars = str.char_indices();
        let mut first = 0;
        let mut second = 0;
        'first: loop {
            match chars.next() {
                Some(val) => {
                    if val.1.is_numeric() {
                        first = val.1.to_digit(10).unwrap();
                        second = first;
                        break;
                    } else {
                        for i in nums.keys() {
                            if val.0 + i.len() > str.len() {
                                continue;
                            }
                            if str[val.0..(val.0 + i.len())].eq(i.to_owned()) {
                                first = nums.get(i).unwrap().to_owned();
                                second = first;
                                break 'first;
                            }
                        }
                    }
                },
                None => {
                    break;
                }
            }
        }
        let mut chars = chars.rev();
        'second: loop {
            match chars.next() {
                Some(val) => {
                    if val.1.is_numeric() {
                        second = val.1.to_digit(10).unwrap();
                        break;
                    } else {
                        for i in nums.keys() {
                            if i.len() + val.0 > str.len() {
                                continue;
                            }
                            if str[(val.0)..(val.0 + i.len())].eq(i.to_owned()) {
                                second = nums.get(i).unwrap().to_owned();
                                break 'second;
                            }
                        }
                    }
                },
                None => {
                    break;
                }
            }
        }
        first * 10 + second
    }
    pub fn test(num: u32, test_str: &str) -> u32 {
        let strings: Vec<&str> = test_str.split("\n").collect();
        let mut sum = 0;
        for i in strings {
            match &num {
                1 => {
                    sum += &number_from_string_first(i);
                }
                2 => {
                    sum += &number_from_string_second(i);
                }
                _ => {}
            }
        }
        sum
    }
}