mod card;

use card::Card;
use std::collections::HashMap;
use std::error::Error;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;

    let cards: Vec<Card> = buffer
        .lines()
        .map(|x| x.parse())
        .collect::<Result<Vec<Card>, _>>()?;

    let part1 = get_part1(&cards);
    println!("Part 1: {}", part1);

    let part2 = get_part2(&cards);
    println!("Part 2: {}", part2);

    Ok(())
}

fn get_part1(cards: &[Card]) -> u32 {
    cards.iter().map(|x| x.get_points()).sum()
}

fn get_part2(cards: &[Card]) -> u32 {
    let mut card_map: HashMap<u32, usize> = HashMap::new();
    for card in cards {
        let copies = *card_map.entry(card.id).or_insert(1);
        let win_count = card.win_count();
        for index in 1..win_count + 1 {
            let key = card.id + (index as u32);
            let value = card_map.entry(key).or_insert(1);
            *value += copies;
        }
    }
    card_map.values().map(|x| *x as u32).sum::<u32>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() -> Result<(), Box<dyn Error>> {
        let example = [
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ];
        let cards: Vec<Card> = example
            .iter()
            .map(|x| x.parse())
            .collect::<Result<Vec<Card>, _>>()?;

        let points: Vec<u32> = cards.iter().map(|x| x.get_points()).collect();

        assert_eq!(points, [8, 2, 2, 1, 0, 0]);
        let part1 = get_part1(&cards);
        assert_eq!(part1, 13);

        let part2 = get_part2(&cards);
        assert_eq!(part2, 30);

        Ok(())
    }
}
