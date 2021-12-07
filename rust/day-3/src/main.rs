use std::error::Error;
use std::fs;

type Result<T> = std::result::Result<T, std::boxed::Box<dyn Error>>;

fn main() -> Result<()> {
    let input_text = fs::read_to_string("input.txt")?;

    // Part 1
    let (gamma_rate, epsilon_rate) = part_one(&input_text);

    println!("gamma={:?}", gamma_rate);
    println!("epsilon={:?}", epsilon_rate);
    println!("power={:?}", gamma_rate * epsilon_rate);

    // Part 2
    let (oxygen_rating, co2_rating) = part_2(&input_text);

    println!("oxygen rating={:?}", oxygen_rating);
    println!("CO2 rating={:?}", co2_rating);
    println!("life support rating={:?}", oxygen_rating * co2_rating);

    Ok(())
}

fn part_one(input_text: &str) -> (u32, u32) {
    let half_lines = input_text.lines().count() as u32 / 2;
    let column_sum: Vec<u32> = input_text
        .lines()
        .map(parse_line)
        .reduce(|column_sum, row| {
            column_sum
                .iter()
                .zip(row.iter())
                .map(|(&a, &b)| a + b)
                .collect()
        })
        .unwrap();
    let gamma_rate_binary: Vec<bool> = column_sum.iter().map(|&x| x > half_lines).collect();
    let epsilon_rate_binary: Vec<bool> = gamma_rate_binary.iter().map(|&x| !x).collect();

    let gamma_rate = binary_vector_to_int(&gamma_rate_binary);
    let epsilon_rate = binary_vector_to_int(&epsilon_rate_binary);

    (gamma_rate, epsilon_rate)
}

fn part_2(input_text: &str) -> (u32, u32) {
    let mut lines: Vec<&str> = input_text.lines().collect();
    lines.sort_unstable();

    let oxygen_rating_binary = find_life_support_rating(&lines, true);
    let oxygen_rating = binary_vector_to_int(&oxygen_rating_binary);

    let co2_rating_binary = find_life_support_rating(&lines, false);
    let co2_rating = binary_vector_to_int(&co2_rating_binary);

    (oxygen_rating, co2_rating)
}

fn find_life_support_rating(lines: &[&str], select_most_common: bool) -> Vec<bool> {
    let line_len = lines[0].len();

    let mut lo: usize = 0;
    let mut hi: usize = lines.len() - 1;

    for i in 0..line_len {
        if lo == hi {
            break;
        }
        let slice = &lines[lo..hi];

        let mid = ((lo + hi) as f64 / 2.0).ceil() as usize;
        let cut = lo + slice.partition_point(|&s| s.chars().nth(i).unwrap() == '0');

        let most_common_char = lines[mid].chars().nth(i).unwrap();
        let top_slice = ((most_common_char == '0') && select_most_common)
            || (most_common_char == '1' && !select_most_common);

        match top_slice {
            true => hi = cut - 1,
            false => lo = cut,
        }
    }

    parse_line(lines[lo]).iter().map(|&x| x == 1).collect()
}

fn parse_line(line: &str) -> Vec<u32> {
    const RADIX: u32 = 10;
    line.chars().map(|c| c.to_digit(RADIX).unwrap()).collect()
}

fn binary_vector_to_int(vec: &[bool]) -> u32 {
    vec.iter().rev().enumerate().fold(
        0u32,
        |s, (n, &bit)| {
            if bit {
                s + 2u32.pow(n as u32)
            } else {
                s
            }
        },
    )
}
