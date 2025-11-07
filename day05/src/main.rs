mod almanac;

use almanac::Almanac;
use std::error::Error;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();

    std::io::stdin().read_to_string(&mut buffer)?;

    let almanac: Almanac = buffer.parse()?;

    let part1 = get_part1(&almanac)?;
    println!("Part 1: {}", part1);

    Ok(())
}

fn get_part1(almanac: &Almanac) -> Result<u64, Box<dyn Error>> {
    Ok(almanac
        .map_all_seeds_to_location()?
        .into_iter()
        .min()
        .ok_or("no data")?)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_example() -> Result<(), Box<dyn Error>> {
        let example = [
            "seeds: 79 14 55 13",
            "",
            "seed-to-soil map:",
            "50 98 2",
            "52 50 48",
            "",
            "soil-to-fertilizer map:",
            "0 15 37",
            "37 52 2",
            "39 0 15",
            "",
            "fertilizer-to-water map:",
            "49 53 8",
            "0 11 42",
            "42 0 7",
            "57 7 4",
            "",
            "water-to-light map:",
            "88 18 7",
            "18 25 70",
            "",
            "light-to-temperature map:",
            "45 77 23",
            "81 45 19",
            "68 64 13",
            "",
            "temperature-to-humidity map:",
            "0 69 1",
            "1 0 69",
            "",
            "humidity-to-location map:",
            "60 56 37",
            "56 93 4",
        ]
        .join("\n");

        let almanac: Almanac = example.parse()?;
        let locations = almanac.map_all_seeds_to_location()?;
        assert_eq!(locations, [82, 43, 86, 35,]);
        let result = get_part1(&almanac)?;

        assert_eq!(result, 35);

        Ok(())
    }
}
