use std::fs;
use std::error::Error;

type Result<T> = std::result::Result<T, std::boxed::Box<dyn Error>>;


pub fn main() -> Result<()> {
    let data = read_lines_as_i32("input.txt")?;

    // Part 1
    let diffs = compute_diffs(&data);
    let positive_count = count_positive(&diffs);
    println!("{}", positive_count);

    // Part 2
    let rolling_sum = compute_rolling_sum(&data, 3);
    let diffs = compute_diffs(&rolling_sum);
    let positive_count = count_positive(&diffs);
    println!("{}", positive_count);
    Ok(())
}


fn read_lines_as_i32(filename: &str) -> Result<Vec<i32>> {
    let file_contents = fs::read_to_string(filename)?;
    let result = file_contents
        .split('\n')
        .filter(|&s| !s.is_empty())
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    Ok(result)
}


fn compute_diffs(data: &[i32]) -> Vec<i32> {
    data
        .windows(2)
        .map(|t| t[1] - t[0])
        .collect()
}


fn count_positive(vec: &[i32]) -> usize {
    vec
        .iter()
        .filter(|&&t| t > 0)
        .count()
}


fn compute_rolling_sum(data: &[i32], window_size: usize) -> Vec<i32> {
    data
        .windows(window_size)
        .map(|t| t.iter().sum())
        .collect()
}

