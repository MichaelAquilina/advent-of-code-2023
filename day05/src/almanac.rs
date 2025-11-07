use std::collections::HashMap;
use std::error::Error;

#[derive(Debug)]
pub struct Almanac {
    seeds: Vec<u64>,
    maps: HashMap<String, Vec<AlmanacMap>>,
}

#[derive(Debug)]
struct AlmanacMap {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

impl Almanac {
    pub fn map_all_seeds_to_location(&self) -> Result<Vec<u64>, Box<dyn Error>> {
        self.seeds
            .iter()
            .map(|x| self.map_seed_to_location(*x))
            .collect()
    }

    fn map_seed_to_location(&self, seed: u64) -> Result<u64, Box<dyn Error>> {
        const MAP_STEPS: [&str; 7] = [
            "seed-to-soil",
            "soil-to-fertilizer",
            "fertilizer-to-water",
            "water-to-light",
            "light-to-temperature",
            "temperature-to-humidity",
            "humidity-to-location",
        ];
        let mut current = seed as i64;

        for step in MAP_STEPS {
            let map_ranges = self.maps.get(step).ok_or(format!("missing {} map", step))?;

            let mut next = current;
            for map_range in map_ranges {
                let destination_range_start = map_range.destination_range_start as i64;
                let source_range_start = map_range.source_range_start as i64;
                let range_length = map_range.range_length as i64;
                let difference: i64 = current - source_range_start;
                if difference >= 0 && difference < range_length {
                    next = destination_range_start + difference;
                    break;
                }
            }
            current = next;
        }

        Ok(current as u64)
    }
}

fn get_map(map_section: &str) -> Result<(String, Vec<AlmanacMap>), Box<dyn Error>> {
    let mut maps: Vec<AlmanacMap> = vec![];
    let mut lines = map_section.lines();
    let name = lines
        .next()
        .ok_or("Missing map name")?
        .strip_suffix(" map:")
        .ok_or("Missing map suffix")?
        .to_string();

    for map_entry in lines {
        let mut tokens = map_entry.split(' ');
        let destination_range_start: u64 = tokens
            .next()
            .ok_or("missing destination range start")?
            .parse()?;
        let source_range_start: u64 = tokens.next().ok_or("missing source range start")?.parse()?;
        let range_length: u64 = tokens.next().ok_or("missing range length")?.parse()?;

        maps.push(AlmanacMap {
            destination_range_start,
            source_range_start,
            range_length,
        });
    }

    Ok((name, maps))
}

impl std::str::FromStr for Almanac {
    type Err = Box<dyn Error>;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut tokens = value.split("\n\n");

        let seeds: Vec<u64> = tokens
            .next()
            .ok_or("Missing seeds")?
            .strip_prefix("seeds: ")
            .ok_or("Missing seeds prefix")?
            .split(' ')
            .map(|x| x.parse())
            .collect::<Result<Vec<u64>, _>>()?;

        let mut maps: HashMap<String, Vec<AlmanacMap>> = HashMap::new();
        for map_section in tokens {
            let (name, map) = get_map(map_section)?;
            maps.insert(name, map);
        }

        Ok(Almanac { seeds, maps })
    }
}
