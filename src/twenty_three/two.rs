///
/// Day Two Problem: Find minimum number of objects needed for a series of collections to occur
/// 
/// Solution: Both cases utilize a function which gets the maximum number of an object type
/// in a list by iterating over each game and each color to get the numeric values. These are
/// then appropriately manipulated for the desired results.
/// 
pub mod day_two {
    use std::{cmp::max, collections::HashMap};
    fn get_max_cubes(game: &str) -> HashMap<String, u32> {
        let games: Vec<&str> = game.split(";").collect();
        let mut cubes = HashMap::new();
        let colors = ["red", "blue", "green"];
        for i in colors {
            let mut color_max = 0;
            for j in 0..games.len() {
                let mut num_chars = games[j].split(i).collect::<Vec<&str>>()[0].chars().rev();
                num_chars.next();
                let mut str = String::new();
                loop {
                    let value = num_chars.next().unwrap();
                    if value.is_numeric() {
                        str = format!("{value}{str}");
                    }
                    else {
                        break;
                    }
                }
                if str.len() > 0 {
                    let num = str.parse::<u32>().unwrap();
                    color_max = max(num, color_max);
                }
            }
            cubes.insert(String::from(i), color_max);
        }
        cubes
    }
    fn cube_game_one(data: &str) -> u32 {
        let games: Vec<&str> = data.split("\n").collect();
        let mut game_number = 1;
        let mut sum = 0;
        for i in games {
            let cubes = get_max_cubes(i);
            if *cubes.get("red").unwrap() <= 12 && *cubes.get("blue").unwrap() <= 14 && *cubes.get("green").unwrap() <= 13 {
                sum += game_number;
            }
            game_number += 1;
        }
        sum
    }
    fn cube_game_two(data: &str) -> u32 {
        let games: Vec<&str> = data.split("\n").collect();
        let mut sum = 0;
        for i in games {
            let cubes = get_max_cubes(i);
            sum += cubes.get("red").unwrap() * cubes.get("blue").unwrap() * cubes.get("green").unwrap();
        }
        sum
    }
    pub fn test(data: &str) -> (u32, u32) {
        (cube_game_one(data), cube_game_two(data))
    }
}