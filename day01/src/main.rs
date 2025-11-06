use std::error::Error;
use std::io::Read;

const NUMBERS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();

    std::io::stdin().read_to_string(&mut buffer)?;

    let part1 = get_part1(&buffer);
    println!("Part 1: {:?}", part1);

    let part2 = get_part2(&buffer);
    println!("Part 2: {:?}", part2);

    Ok(())
}

fn get_part1(buffer: &str) -> u32 {
    buffer
        .lines()
        .map(|x| get_calibration_value(x, false))
        .sum()
}

fn get_part2(buffer: &str) -> u32 {
    buffer.lines().map(|x| get_calibration_value(x, true)).sum()
}

fn get_calibration_value(line: &str, spelled: bool) -> u32 {
    let mut digits: Vec<u32> = vec![];

    for (index, token) in line.char_indices() {
        if token.is_ascii_digit() {
            digits.push(token.to_digit(10).unwrap());
        } else if spelled {
            let slice = &line[index..];
            for (index2, number) in NUMBERS.iter().enumerate() {
                if slice.starts_with(number) {
                    digits.push(index2 as u32);
                    break;
                }
            }
        }
    }

    let last = digits.last().unwrap();
    let first = digits.first().unwrap();
    format!("{}{}", first, last).parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let example = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"].join("\n");
        let result = get_part1(&example);
        assert_eq!(result, 142);
    }

    #[test]
    fn test_get_part2_example() {
        let example = [
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ]
        .join("\n");
        let result = get_part2(&example);
        assert_eq!(result, 281);
    }
}
