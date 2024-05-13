///
/// Day Eight Problem: Follow a map from start points and count steps
/// 
/// Solution: Both parts impliment a similar loop of following directions until
/// arriving at the desired result. The second part does this for each start point
/// and finds the minimum number of steps for each location, then takes the LCM of
/// those step values to find the minimum number of steps for all the paths to
/// simultaniously arrive at the desired values.
/// 
pub mod day_eight {
    use std::collections::HashMap;

    #[derive(Debug)]
    struct Map {
        directions: String,
        map: HashMap<String, (String, String)>
    }

    impl Map {
        fn steps_to_exit(&self) -> usize {
            let mut current_location = "AAA";
            let mut steps = 0;
            let directions: Vec<char> = self.directions.chars().collect();
            while !current_location.eq("ZZZ") {
                let new_locations = self.map.get(current_location).unwrap();
                if directions[steps % directions.len()].eq(&'L') {
                    current_location = &new_locations.0;
                } else {
                    current_location = &new_locations.1
                }
                steps += 1;
            }
            steps
        }
    
        fn paths_to_exit(&self, starts: Vec<&str>) -> usize {
            let current_locations = starts;
            let directions: Vec<char> = self.directions.chars().collect();
            let mut location_steps = Vec::new();
            for mut i in current_locations {
                let mut steps = 0;
                while !i.chars().nth(2).unwrap().eq(&'Z') {
                    let new_locations: &(String, String) = self.map.get(i).unwrap();
                    if directions[steps % directions.len()].eq(&'L') {
                        i = &new_locations.0;
                    } else {
                        i = &new_locations.1
                    }
                    steps += 1;
                }
                location_steps.push(steps);
            }
            let mut max = *location_steps.iter().max().unwrap();
            let mut lcm = max;
            for i in location_steps {
                while lcm % i != 0 {
                    lcm += max;
                }
                max = lcm;
            }
            lcm
        }
    }

    fn parts(string: &str) -> (usize, usize) {
        let parts: Vec<&str> = string.split("\n\n").collect();
        let directions = String::from(parts[0]);
        let mut map = HashMap::new();
        let cells: Vec<&str> = parts[1].split('\n').collect();
        let mut starts = Vec::new();
        for i in cells {
            if i[2..3].eq("A") {
                starts.push(&i[0..3]);
            }
            map.insert(String::from(&i[0..3]), (String::from(&i[7..10]), String::from(&i[12..15])));
        }
        let map = Map {directions, map};
        (map.steps_to_exit(), map.paths_to_exit(starts))
    }


    pub fn test(string: &str) -> (usize, usize) {
        parts(string)
    }
}