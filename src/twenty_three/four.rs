///
/// Day Four Problem: Read and count matching numbers between two sets
/// 
/// Solution: Both parts leverage a utility function to aquire the number of winning numbers
/// in any card. Then both parts appropriately manipulate the wins data to produce the desired
/// result, be it points in the first, or cards totaled in the second.
/// 
pub mod day_four {
    fn winning_numbers(string: &str) -> usize {
        let card: Vec<&str> = string.split(':').collect::<Vec<&str>>().last().unwrap_or(&"|").split('|').collect();
        if card.len() != 2 { return 0 }
        let mut winning_numbers = 0;
        let winners: Vec<usize> = card[0].split_whitespace().map(|x| x.parse::<usize>().unwrap_or(0)).collect();
        let nums: Vec<usize> = card[1].split_whitespace().map(|x| x.parse::<usize>().unwrap_or(0)).collect();
        for i in nums {
            for j in &winners {
                if i == *j {
                    winning_numbers += 1;
                }
            }
        }
        winning_numbers
    }

    fn points(collection: &str) -> usize {
        let cards: Vec<&str> = collection.split('\n').collect();
        let mut points = 0;
        for i in cards {
            let mut wins = winning_numbers(i);
            if wins > 0 {
                let mut win_points = 1;
                while wins > 1 {
                    win_points *= 2;
                    wins -= 1;
                }
                points += win_points;
            }
        }
        points
    }

    fn cards(collection: &str) -> usize {
        let cards: Vec<&str> = collection.split('\n').collect();
        let mut card_map = vec![1; cards.len()];
        let mut it = cards.iter();
        let mut index = 0;
        loop {
            match it.next() {
                Some(k) => {
                    let wins = winning_numbers(*k);
                    for i in 1..wins + 1 {
                        card_map[index + i] += card_map[index];
                    }
                    index += 1;
                },
                None => {break;}
            }
        }
        card_map.iter().sum()
    }

    pub fn test(string: &str) -> (usize, usize) {
        (points(string), cards(string))
    }
}