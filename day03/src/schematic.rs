use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct Schematic {
    pub numbers: HashMap<Point, u32>,
    pub symbols: HashMap<Point, char>,
}

impl Schematic {
    pub fn is_part_number(&self, point: &Point) -> bool {
        for neighbor in self.get_neighbors(point) {
            if self.get_symbol(&neighbor).is_some() {
                return true;
            }
        }
        false
    }

    pub fn get_neighbors(&self, point: &Point) -> Vec<Point> {
        let mut neighbors = vec![];
        let x = point.x;
        let y = point.y;
        if let Some(number) = self.numbers.get(point) {
            let length = number.to_string().len();
            neighbors.extend([
                Point { x: x - 1, y: y - 1 },
                Point { x: x - 1, y },
                Point { x: x - 1, y: y + 1 },
            ]);
            for index in 0..length {
                let x = x + (index as i32);
                neighbors.push(Point { x, y: y - 1 });
                neighbors.push(Point { x, y: y + 1 });
            }
            let x = x + (length as i32);
            neighbors.extend([Point { x, y: y - 1 }, Point { x, y }, Point { x, y: y + 1 }])
        }

        neighbors
    }

    pub fn get_symbol(&self, point: &Point) -> Option<&char> {
        self.symbols.get(point)
    }
}

impl std::str::FromStr for Schematic {
    type Err = Box<dyn Error>;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut numbers: HashMap<Point, u32> = HashMap::new();
        let mut symbols: HashMap<Point, char> = HashMap::new();

        for (y, line) in value.lines().enumerate() {
            let mut number_tokens: Vec<char> = vec![];
            let mut point: Option<Point> = None;
            for (x, token) in line.char_indices() {
                let x = x as i32;
                let y = y as i32;
                if token.is_ascii_digit() {
                    number_tokens.push(token);
                    point.get_or_insert(Point { x, y });
                } else {
                    if let Some(key) = point {
                        let number: u32 = number_tokens.iter().collect::<String>().parse()?;
                        numbers.entry(key).or_insert(number);

                        point = None;
                        number_tokens.clear();
                    }

                    if token != '.' {
                        let point = Point { x, y };
                        symbols.entry(point).or_insert(token);
                    }
                }
            }
        }
        Ok(Schematic { numbers, symbols })
    }
}
