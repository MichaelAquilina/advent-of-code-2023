mod schematic;

use schematic::Schematic;
use std::error::Error;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();

    std::io::stdin().read_to_string(&mut buffer)?;
    let schematic: Schematic = buffer.parse()?;

    let part1 = get_part1(&schematic);
    println!("Part 1: {}", part1);

    Ok(())
}

fn get_part1(schematic: &Schematic) -> u32 {
    let mut total = 0;
    for (point, number) in schematic.numbers.iter() {
        if schematic.is_part_number(point) {
            total += number;
        }
    }
    total
}

#[cfg(test)]
mod test {
    use super::*;
    use schematic::Point;

    #[test]
    fn test_corner_case() -> Result<(), Box<dyn Error>> {
        let example = ["12..+..", "..34...", "..@56..", "78.....", "......."].join("\n");

        let schematic: Schematic = example.parse()?;
        let result = get_part1(&schematic);
        assert_eq!(result, 168);

        Ok(())
    }

    #[test]
    fn test_no_part_symbols() -> Result<(), Box<dyn Error>> {
        let example = ["...", ".6.", "..."].join("\n");
        let schematic: Schematic = example.parse()?;
        let result = get_part1(&schematic);
        assert_eq!(result, 0);

        Ok(())
    }

    #[test]
    fn test_part_symbols() -> Result<(), Box<dyn Error>> {
        let example = ["fdwdawd", "f82+18o", "fhello+"].join("\n");
        let schematic: Schematic = example.parse()?;
        let result = get_part1(&schematic);
        assert_eq!(result, 100);

        Ok(())
    }

    #[test]
    fn test_part1_example() -> Result<(), Box<dyn Error>> {
        let example = [
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ]
        .join("\n");
        let schematic: Schematic = example.parse()?;
        let result = get_part1(&schematic);
        assert_eq!(result, 4361);

        Ok(())
    }

    #[test]
    fn test_parse_schematic() -> Result<(), Box<dyn Error>> {
        let example = [
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ]
        .join("\n");
        let schematic: Schematic = example.parse()?;
        let point = Point { x: 0, y: 1 };
        assert_eq!(schematic.get_neighbors(&point), vec![]);
        let point = Point { x: 0, y: 0 };
        let neighbors = schematic.get_neighbors(&point);
        assert_eq!(schematic.symbols.len(), 6);
        assert_eq!(schematic.numbers.len(), 10);
        assert_eq!(
            neighbors,
            [
                Point { x: -1, y: -1 },
                Point { x: -1, y: 0 },
                Point { x: -1, y: 1 },
                Point { x: 0, y: -1 },
                Point { x: 0, y: 1 },
                Point { x: 1, y: -1 },
                Point { x: 1, y: 1 },
                Point { x: 2, y: -1 },
                Point { x: 2, y: 1 },
                Point { x: 3, y: -1 },
                Point { x: 3, y: 0 },
                Point { x: 3, y: 1 },
            ]
        );

        Ok(())
    }
}
