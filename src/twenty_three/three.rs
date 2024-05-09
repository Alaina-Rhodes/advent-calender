///
/// Day Three Problem: Use adjacent characters to determine a value from a map
/// 
/// Solution: In the first solution where a number must be adjacent to a symbol, a collection
/// of strings was acquired based on the appropriate index range of that number and searched
/// for symbols. In the second solution, the gears (*) are found and from there a range for
/// each row which includes the relevant digits.
/// 
/// Limitations: This function will not work for maps of characters with differernt numbers
/// of bytes used (i.e. if there are some characters which use two bytes these functions fail)
/// 
pub mod day_three {
    fn includes_symbol(string: &str) -> bool {
        if string.contains(&['*', '#', '$', '%', '&', '@', '-', '+', '=', '/']) {
            return true;
        }
        false
    }

    fn part_numbers(string: &str) -> u32 {
        let rows: Vec<&str> = string.split('\n').collect();
        let mut sum = 0;
        for i in 0..rows.len() {
            let mut chars = rows[i].char_indices();
            loop {
                match chars.next() {
                    Some(x) => {
                        let (index, char) = x;
                        let mut len = 1;
                        if char.is_numeric() {
                            while chars.next().unwrap_or((0, '.')).1.is_numeric() {
                                len += 1;
                            }
                            let lower_bound = if index == 0 {0} else {index - 1};
                            let upper_bound = if index + len + 1 > rows[i].len() {rows[i].len()} else {index + len + 1};
                            if includes_symbol(&rows[i][lower_bound..upper_bound]) ||
                               i > 0 && includes_symbol(&rows[i - 1][lower_bound..upper_bound]) ||
                               i < rows.len() - 1 && includes_symbol(&rows[i + 1][lower_bound..upper_bound]) {
                                sum += &rows[i][index..index + len].parse::<u32>().unwrap_or(0);
                            }
                            
                        }
                    }
                    None => {
                        break;
                    }
                }
            }
        }
        sum
    }

    fn gear_numbers(string: &str) -> u32 {
        let rows: Vec<&str> = string.split("\n").collect();
        let mut sum = 0;
        for i in 0..rows.len() {
            let mut chars = rows[i].char_indices();
            loop {
                match chars.next() {
                    Some(x) => {
                        let (index, char) = x;
                        if char.eq(&'*') {
                            let mut nums: Vec<u32> = vec![];
                            let mut check_rows = vec![i];
                            if i > 0 { check_rows.push(i - 1) }
                            if i < rows.len() - 1 { check_rows.push(i + 1) }
                            for j in check_rows {
                                let mut indices = (if index > 0 {index} else {0}, if index < rows[i].len() - 1 {index + 1} else {rows[i].len()});
                                while indices.0 != 0 && rows[j][indices.0 - 1..indices.0].chars().next().unwrap_or('.').is_numeric() {
                                    indices.0 -= 1;
                                }
                                while indices.1 != rows[j].len() && rows[j][indices.1..indices.1 + 1].chars().next().unwrap_or('.').is_numeric() {
                                    indices.1 += 1;
                                }
                                let terms: Vec<&str> = rows[j][indices.0..indices.1].split(&['.', '*', '#', '$', '%', '&', '@', '-', '+', '=', '/']).collect();
                                for k in terms {
                                    match k.parse::<u32>() {
                                        Ok(num) => nums.push(num),
                                        Err(_) => {}
                                    }
                                }
                            }
                            if nums.len() == 2 {
                                sum += nums[0] * nums[1];
                            }
                        }
                    },
                    None => {break;}
                }
            }
        }
        sum
    }
    pub fn test (string: &str) -> (u32, u32) {
        (part_numbers(string), gear_numbers(string))
    }
}