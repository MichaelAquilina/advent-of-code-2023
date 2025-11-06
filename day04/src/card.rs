use std::collections::HashSet;
use std::error::Error;

#[derive(Debug)]
pub struct Card {
    pub id: u32,
    pub numbers: HashSet<u32>,
    pub winning_numbers: HashSet<u32>,
}

impl Card {
    pub fn win_count(&self) -> usize {
        self.numbers.intersection(&self.winning_numbers).count()
    }

    pub fn get_points(&self) -> u32 {
        let overlap = self.win_count() as u32;
        let base: u32 = 2;
        match overlap {
            0 => 0,
            _ => base.pow(overlap - 1),
        }
    }
}

impl std::str::FromStr for Card {
    type Err = Box<dyn Error>;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut tokens = value.split(": ");
        let id = tokens
            .next()
            .ok_or("Missing card id")?
            .strip_prefix("Card ")
            .ok_or("Missing card prefix")?
            .trim_start()
            .parse()?;

        tokens = tokens
            .next()
            .ok_or("Missing any number values")?
            .split(" | ");

        let numbers: HashSet<u32> = tokens
            .next()
            .ok_or("Missing numbers")?
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.parse())
            .collect::<Result<HashSet<u32>, _>>()?;

        let winning_numbers: HashSet<u32> = tokens
            .next()
            .ok_or("Missing winning numbers")?
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.parse())
            .collect::<Result<HashSet<u32>, _>>()?;

        Ok(Card {
            id,
            numbers,
            winning_numbers,
        })
    }
}
