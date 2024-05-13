///
/// Day Nine Problem: Find previous and next numbers in a pattern
/// 
/// Solution: Using patterns from polynomial fitting, we add the differences in
/// rates to get the next or subtract to get the previous using the coresponding
/// values.
/// 
pub mod day_nine {
    fn empty_row(row: &Vec<i64>) -> bool {
        for i in row {
            if !(*i == 0) {
                return false;
            }
        }
        true
    }
    
    fn next_term(history: &str) -> i64 {
        let mut rows: Vec<Vec<i64>> = vec![history.split(' ').collect::<Vec<&str>>().iter().flat_map(|num| num.parse::<i64>()).collect()];
        if empty_row(rows.last().unwrap()) { return 0; }
        loop {
            let mut new_row = Vec::new();
            let last_row = rows.last().unwrap();
            for i in 0..last_row.len() - 1 {
                new_row.push(last_row[i + 1] - last_row[i]);
            }
            if empty_row(&new_row) { break; }
            rows.push(new_row);
        }
        let mut term = 0;
        for i in rows {
            term += i.last().unwrap_or(&0);
        }
        term
    }

    fn prev_term(history: &str) -> i64 {
        let mut rows: Vec<Vec<i64>> = vec![history.split(' ').collect::<Vec<&str>>().iter().flat_map(|num| num.parse::<i64>()).collect()];
        if empty_row(rows.last().unwrap()) { return 0; }
        loop {
            let mut new_row = Vec::new();
            let last_row = rows.last().unwrap();
            for i in 0..last_row.len() - 1 {
                new_row.push(last_row[i + 1] - last_row[i]);
            }
            if empty_row(&new_row) { rows.push(new_row); break; }
            rows.push(new_row);
        }
        let mut term = 0;
        let rows_len = rows.len();
        for i in 2..rows_len + 1 {
            term = rows[rows_len - i][0] - term
        }
        term
    }

    fn parts(string: &str) -> (i64, i64) {
        let histories: Vec<&str> = string.split('\n').collect();
        let mut next = 0;
        let mut prev = 0;
        for i in histories {
            next += next_term(i);
            prev += prev_term(i);
        }
        (next, prev)
    }

    pub fn test(string: &str) -> (i64, i64) {
        parts(string)
    }
}