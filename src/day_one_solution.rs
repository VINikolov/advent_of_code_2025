use itertools::Itertools;
use std::fs;

fn get_new_current_result(value: &str, current: i32) -> i32 {
    let dir = match value.chars().nth(0).unwrap() {
        'L' => -1,
        _ => 1,
    };
    let move_value = &value[1..].parse::<i32>().unwrap();
    let mut result = current;

    result += dir * move_value;

    return result % 100;
}

fn solve_part_one(input: &Vec<&str>) -> i32 {
    let mut current = 50;
    let mut result = 0;

    input.iter().for_each(|value| {
        current = get_new_current_result(value, current);

        if current == 0 {
            result += 1;
        }
    });

    return result;
}

pub fn solve_day_one() -> (i32, i32) {
    let input_file = fs::read_to_string("./src/input_files/day_one.txt").unwrap();
    let input = input_file.lines().collect_vec();

    return (solve_part_one(&input), 0);
}
