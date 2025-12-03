use std::fs;

fn get_sum_of_invalid_ids(range: &str) -> i64 {
    let (start, end) = range.trim().split_once("-").unwrap();
    let (start, end): (i64, i64) = (start.parse().unwrap(), end.parse().unwrap());
    let mut result = 0;

    for num in start..=end {
        let num_str = num.to_string();
        let half_len = num_str.len() / 2;

        if &num_str[0..half_len] == &num_str[half_len..] {
            result += num;
        }
    }

    return result;
}

fn solve_part_one(input: &str) -> i64 {
    return input
        .split(",")
        .map(|range| get_sum_of_invalid_ids(range))
        .sum();
}

pub fn solve_day_two() -> (i64, i64) {
    let input = fs::read_to_string("./src/input_files/day_two.txt").unwrap();

    return (solve_part_one(&input), 0);
}
