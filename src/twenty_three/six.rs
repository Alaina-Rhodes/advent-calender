pub mod day_six {
    fn record_races_total(seconds: usize, distance: usize) -> usize {
        let mut seconds_held = 1;
        while (!(seconds_held * (seconds - seconds_held) > distance) && seconds_held < seconds / 2 + 1) {
            seconds_held += 1;
        }
        seconds - 2 * seconds_held + 1
    }

    fn part_one(string: &str) -> usize {
        let params: Vec<&str> = string.split('\n').collect();
        let times: Vec<usize> = params[0].split_whitespace().flat_map(|x| x.parse::<usize>()).collect();
        let distances: Vec<usize> = params[1].split_whitespace().flat_map(|x| x.parse::<usize>()).collect();
        let mut combinations = 1;
        if times.len() != distances.len() {
            return 0
        }
        for i in 0..times.len() {
            combinations *= record_races_total(times[i], distances[i]);
        }
        combinations
    }

    fn part_two(string: &str) -> usize {
        let params: Vec<&str> = string.split('\n').collect();
        let time = params[0].split(":").nth(1).unwrap_or("0").replace(' ', "");
        let distance = params[1].split(":").nth(1).unwrap_or("0").replace(' ', "");
        record_races_total(time.parse().unwrap_or(0), distance.parse().unwrap_or(0))
    }

    pub fn test(string: &str) -> (usize, usize) {
        (part_one(string), part_two(string))
    }
}