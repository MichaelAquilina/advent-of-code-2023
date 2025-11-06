mod game;

use game::{Game, GameSet};
use std::cmp;
use std::error::Error;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();

    std::io::stdin().read_to_string(&mut buffer)?;

    let games: Vec<Game> = buffer
        .lines()
        .map(|x| x.parse())
        .collect::<Result<Vec<Game>, _>>()?;

    let part1 = get_part1(&games);
    println!("Part 1: {}", part1);

    let part2 = get_part2(&games);
    println!("Part 2: {}", part2);

    Ok(())
}

fn get_part1(games: &[Game]) -> u32 {
    let mut result = 0;
    for game in games {
        if game.game_sets.iter().all(valid_game_set) {
            result += game.id;
        }
    }
    result
}

fn get_part2(games: &[Game]) -> u32 {
    games.iter().map(min_constraints).map(|x| power(&x)).sum()
}

fn valid_game_set(game_set: &GameSet) -> bool {
    game_set.red <= 12 && game_set.green <= 13 && game_set.blue <= 14
}

fn min_constraints(game: &Game) -> GameSet {
    let mut constraints = GameSet {
        red: 0,
        green: 0,
        blue: 0,
    };
    for game_set in &game.game_sets {
        constraints.red = cmp::max(constraints.red, game_set.red);
        constraints.green = cmp::max(constraints.green, game_set.green);
        constraints.blue = cmp::max(constraints.blue, game_set.blue);
    }
    constraints
}

fn power(game_set: &GameSet) -> u32 {
    game_set.red * game_set.green * game_set.blue
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_example() -> Result<(), Box<dyn Error>> {
        let example = [
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];
        let games: Vec<Game> = example
            .iter()
            .map(|x| x.parse())
            .collect::<Result<Vec<Game>, _>>()?;

        let result = get_part1(&games);
        assert_eq!(result, 8);

        Ok(())
    }

    #[test]
    fn test_part2_example() -> Result<(), Box<dyn Error>> {
        let example = [
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];
        let games: Vec<Game> = example
            .iter()
            .map(|x| x.parse())
            .collect::<Result<Vec<Game>, _>>()?;

        let result = get_part2(&games);
        assert_eq!(result, 2286);

        Ok(())
    }
}
