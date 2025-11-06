use std::error::Error;

#[derive(Debug)]
pub struct GameSet {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

#[derive(Debug)]
pub struct Game {
    pub id: u32,
    pub game_sets: Vec<GameSet>,
}

impl std::str::FromStr for GameSet {
    type Err = Box<dyn Error>;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for cubes in value.split(", ") {
            let mut tokens = cubes.split(' ');
            let value: u32 = tokens.next().ok_or("invalid cube value")?.parse()?;
            let color = tokens.next().ok_or("missing cube color")?;
            match color {
                "red" => red = value,
                "green" => green = value,
                "blue" => blue = value,
                x => return Err(format!("Unknown Color {}", x).into()),
            };
        }
        Ok(GameSet { red, green, blue })
    }
}

impl std::str::FromStr for Game {
    type Err = Box<dyn Error>;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut tokens = value.split(": ");
        let id: u32 = tokens
            .next()
            .ok_or("Missing id")?
            .strip_prefix("Game ")
            .ok_or("Missing Game prefix")?
            .parse()?;

        let mut game_sets: Vec<GameSet> = vec![];
        let sets = tokens.next().ok_or("missing set values")?.split("; ");

        for set in sets {
            game_sets.push(set.parse()?);
        }

        Ok(Game { id, game_sets })
    }
}
