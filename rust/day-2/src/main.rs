use std::error::Error;
use std::fs;

type Result<T> = std::result::Result<T, std::boxed::Box<dyn Error>>;
type Position = (i32, i32);

fn main() -> Result<()> {
    let position_diffs = read_and_parse_input("input.txt")?;

    // Part 1
    let final_pos = position_diffs
        .iter()
        .fold((0i32, 0i32), |current_pos, pos_diff| {
            (current_pos.0 + pos_diff.0, current_pos.1 + pos_diff.1)
        });
    println!("{:?} => {}", final_pos, final_pos.0 * final_pos.1);

    // Part 2
    let final_pos = position_diffs
        .iter()
        .fold((0i32, 0i32, 0i32), |current_pos, pos_diff| {
            let aim = current_pos.0 + pos_diff.0;
            let depth = current_pos.1 + pos_diff.1 * aim;
            let horizontal = current_pos.2 + pos_diff.1;
            (aim, depth, horizontal)
        });
    println!("{:?} => {}", final_pos, final_pos.1 * final_pos.2);

    Ok(())
}

fn read_and_parse_input(filename: &str) -> Result<Vec<Position>> {
    let file_contents = fs::read_to_string(filename)?;

    let result = file_contents
        .split('\n')
        .filter(|&s| !s.is_empty())
        .map(parse_command_to_position_diff)
        .collect();

    Ok(result)
}

fn parse_command_to_position_diff(command: &str) -> Position {
    let dir_and_step_size: Vec<&str> = command.split(' ').collect();
    let direction = dir_and_step_size[0];
    let step_size: i32 = dir_and_step_size[1].parse().unwrap();

    if direction == "up" {
        (-step_size, 0)
    } else if direction == "down" {
        (step_size, 0)
    } else {
        (0, step_size)
    }
}
