///
/// Day Five Problem: Map points and ranges to locations
/// 
/// Solution: For the first part used a vector of ranges to follow each individual seed's
/// final location. Second part is incomplete and still giving trouble. Present concept is 
/// to generate a collection of ranges which directly map to the final locations, then take 
/// the minimum possible value of those ranges and get the smallest location that way.
/// Presently works for the small test case, but not the larger case.
/// 
pub mod day_five {
    #[derive(Debug)]
    struct Range {
        min: i64,
        max: i64,
        diff: i64,
    }

    impl Range {
        fn in_range(&self, num: i64) -> bool {
            num >= self.min && num <= self.max
        }

        fn intersects(&self, range: &Range) -> bool {
            self.min + self.diff < range.max && self.max + self.diff > range.min
        }

        fn encapsulated_by(&self, range: &Range) -> bool {
            self.min + self.diff >= range.min && self.max + self.diff <= range.max
        }
    }

    fn smallest_seed(string: &str) -> i64 {
        let mut seed_ranges: Vec<Vec<Range>> = Vec::new();
        let parts: Vec<&str> = string.split("\n\n").collect();
        let seeds: Vec<i64> = parts[0].split(": ").collect::<Vec<&str>>()[1].split(' ').flat_map(|x| x.parse::<i64>()).collect();
        for i in parts {
            let mut ranges = i.split("\n").into_iter();
            let mut new_ranges: Vec<Range> = Vec::new();
            ranges.next();
            loop {
                match ranges.next() {
                    Some(range) => {
                        let nums: Vec<i64> = range.split(' ').collect::<Vec<&str>>().iter().flat_map(|x| x.parse::<i64>()).collect();
                        new_ranges.push(Range {min: nums[1], max: nums[1] + nums[2] - 1, diff: nums[0] - nums[1]});
                    },
                    None => {break;}
                }
            }
            seed_ranges.push(new_ranges);
        }
        let mut locations: Vec<i64> = Vec::new();
        for i in seeds {
            let mut location = i;
            for j in &seed_ranges {
                for k in j {
                    if k.in_range(location) {
                        location += k.diff;
                        break;
                    }
                }
            }
            locations.push(location);
        }
        locations.sort();
        locations[0]
    }

    fn smallest_seed_range(string: &str) -> i64 {
        let mut seed_ranges: Vec<Range> = Vec::new();
        let parts: Vec<&str> = string.split("\n\n").collect();
        let seeds: Vec<i64> = parts[0].split(": ").collect::<Vec<&str>>()[1].split(' ').flat_map(|x| x.parse::<i64>()).collect();
        let mut index = 0;
        while index < seeds.len() {
            seed_ranges.push(Range {min: seeds[index], max: seeds[index] + seeds[index + 1] - 1, diff: 0});
            index += 2;
        }
        for i in parts {
            let mut ranges = i.split("\n").into_iter();
            let mut new_ranges: Vec<Range> = Vec::new();
            ranges.next();
            loop {
                match ranges.next() {
                    Some(range) => {
                        let nums: Vec<i64> = range.split(' ').collect::<Vec<&str>>().iter().flat_map(|x| x.parse::<i64>()).collect();
                        let (min, max, diff) = (nums[1], nums[2] + nums[1] - 1, nums[0] - nums[1]);
                        let new_range = Range {min, max, diff};
                        seed_ranges.retain(|j| {
                            if j.encapsulated_by(&new_range) {
                                new_ranges.push(Range {min: j.min, max: j.max, diff: j.diff + diff});
                                return false
                            }
                            if j.intersects(&new_range) {
                                let range_min = j.min + j.diff;
                                let range_max = j.max + j.diff;
                                if min > range_min {
                                    new_ranges.push(Range {min: j.min, max: min - j.diff - 1, diff: j.diff});
                                 }
                                if max < range_max {
                                    new_ranges.push(Range {min: max - j.diff + 1, max: j.max, diff: j.diff});
                                }
                                let overlap = (if min > range_min { min - j.diff } else { j.min }, if max < range_max { max - j.diff } else { j.max });
                                new_ranges.push(Range {min: overlap.0, max: overlap.1, diff: j.diff + diff});
                                return false
                            }
                            true
                        })
                    },
                    None => {break;}
                }
            }
            seed_ranges.append(&mut new_ranges);
        }
        let mut smallest_seed = seed_ranges[0].min + seed_ranges[0].diff;
        for i in seed_ranges {
            if i.min + i.diff < smallest_seed {
                smallest_seed = i.min + i.diff;
            }
        }
        smallest_seed
    }
    pub fn test(string: &str) -> (i64, i64) {
        (smallest_seed(string), smallest_seed_range(string))
    }
}