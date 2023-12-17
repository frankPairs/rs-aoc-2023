use std::{
    collections::{HashMap, VecDeque},
    fs,
    str::Lines,
};

const MAX_GREEN_CUBES: u32 = 13;
const MAX_RED_CUBES: u32 = 12;
const MAX_BLUE_CUBES: u32 = 14;

#[derive(Debug)]
enum GameError {
    InvalidId,
    InvalidColor(String),
}

impl std::fmt::Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameError::InvalidId => {
                write!(f, "Invalid game id")
            }
            GameError::InvalidColor(color) => {
                write!(f, "Invalid color called {}", color)
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum CubeColor {
    Blue,
    Red,
    Green,
}

impl TryFrom<String> for CubeColor {
    type Error = GameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "blue" => Ok(CubeColor::Blue),
            "red" => Ok(CubeColor::Red),
            "green" => Ok(CubeColor::Green),
            color => Err(GameError::InvalidColor(String::from(color))),
        }
    }
}

#[derive(Debug)]
struct CubeSet {
    cubes: HashMap<CubeColor, u32>,
}

impl CubeSet {
    fn is_valid(&self) -> bool {
        let green_cubes = self.get_quantity_by_color(&CubeColor::Green);
        let red_cubes = self.get_quantity_by_color(&CubeColor::Red);
        let blue_cubes = self.get_quantity_by_color(&CubeColor::Blue);

        green_cubes <= MAX_GREEN_CUBES && red_cubes <= MAX_RED_CUBES && blue_cubes <= MAX_BLUE_CUBES
    }

    fn get_quantity_by_color(&self, color: &CubeColor) -> u32 {
        *self.cubes.get(color).unwrap_or(&0)
    }
}

impl TryFrom<String> for CubeSet {
    type Error = GameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut cubes = HashMap::new();
        let parts: Vec<String> = value.split(",").map(|str| str.to_string()).collect();

        for part in parts {
            let mut cube_parts: VecDeque<String> = part
                .split_ascii_whitespace()
                .map(|str| str.to_string())
                .collect();

            let cube_color: CubeColor =
                CubeColor::try_from(cube_parts.pop_back().unwrap_or(String::from("")))?;
            let cube_quantity: u32 = cube_parts
                .pop_back()
                .unwrap_or(String::from(""))
                .parse()
                .unwrap_or(0);

            cubes.insert(cube_color, cube_quantity);
        }

        Ok(CubeSet { cubes })
    }
}

#[derive(Debug)]
struct Game {
    pub id: u32,
    pub cube_sets: Vec<CubeSet>,
    pub min_set_cubes: HashMap<CubeColor, u32>,
}

impl Game {
    fn is_valid(&self) -> bool {
        self.cube_sets
            .as_slice()
            .into_iter()
            .all(|cube_set| cube_set.is_valid())
    }

    fn power_min_set_of_cubes(&self) -> u32 {
        if self.min_set_cubes.is_empty() {
            return 0;
        }

        let green_min_cubes = self
            .get_min_of_cubes_by_color(&CubeColor::Green)
            .unwrap_or(&1);
        let red_min_cubes = self
            .get_min_of_cubes_by_color(&CubeColor::Red)
            .unwrap_or(&1);
        let blue_min_cubes = self
            .get_min_of_cubes_by_color(&CubeColor::Blue)
            .unwrap_or(&1);

        green_min_cubes * red_min_cubes * blue_min_cubes
    }

    fn get_min_of_cubes_by_color(&self, color: &CubeColor) -> Option<&u32> {
        self.min_set_cubes.get(color)
    }
}

impl TryFrom<String> for Game {
    type Error = GameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut min_set_cubes: HashMap<CubeColor, u32> = HashMap::new();

        let mut parts: VecDeque<String> =
            value.split(":").map(|str| str.trim().to_string()).collect();
        let mut cube_sets: Vec<CubeSet> = vec![];

        let mut game_parts: Vec<String> = parts
            .pop_front()
            .unwrap_or(String::from(""))
            .split_ascii_whitespace()
            .map(|str| str.trim().to_string())
            .collect();
        let cube_sets_parts: Vec<String> = parts
            .pop_front()
            .unwrap_or(String::from(""))
            .split(";")
            .map(|str| str.trim().to_string())
            .collect();

        let game_id: u32 = game_parts
            .pop()
            .unwrap_or(String::from(""))
            .parse()
            .map_err(|_| GameError::InvalidId)?;

        for cube_set_part in cube_sets_parts {
            let cube_set = CubeSet::try_from(cube_set_part)?;
            let green_quantity = cube_set.get_quantity_by_color(&CubeColor::Green);
            let blue_quantity = cube_set.get_quantity_by_color(&CubeColor::Blue);
            let red_quantity = cube_set.get_quantity_by_color(&CubeColor::Red);

            if blue_quantity > *min_set_cubes.get(&CubeColor::Blue).unwrap_or(&0) {
                min_set_cubes.insert(CubeColor::Blue, blue_quantity);
            }

            if red_quantity > *min_set_cubes.get(&CubeColor::Red).unwrap_or(&0) {
                min_set_cubes.insert(CubeColor::Red, red_quantity);
            }

            if green_quantity > *min_set_cubes.get(&CubeColor::Green).unwrap_or(&0) {
                min_set_cubes.insert(CubeColor::Green, green_quantity);
            }

            cube_sets.push(cube_set);
        }

        Ok(Game {
            id: game_id,
            cube_sets,
            min_set_cubes,
        })
    }
}

/// It sums the ids of the valid games.
fn sum_valid_games_from_lines(lines: Lines) -> u32 {
    lines.fold(0, |acc, line| {
        let game = Game::try_from(line.to_string()).expect("Invalid game");

        if game.is_valid() {
            acc + game.id
        } else {
            acc
        }
    })
}

fn sum_of_power_of_min_set_of_cubes_from_lines(lines: Lines) -> u32 {
    lines.fold(0, |acc, line| {
        let game = Game::try_from(line.to_string()).expect("Invalid game");

        acc + game.power_min_set_of_cubes()
    })
}

fn main() {
    let input_str = fs::read_to_string("input.txt");

    match input_str {
        Ok(content) => {
            let lines: Lines = content.lines();

            let sum_valid_games_ids = sum_valid_games_from_lines(lines.clone());
            let sum_of_power_of_min_set_of_cubes =
                sum_of_power_of_min_set_of_cubes_from_lines(lines.clone());

            println!("Total sum of valid games =  {}", sum_valid_games_ids);
            println!(
                "Total sum of power of min set of cubes =  {}",
                sum_of_power_of_min_set_of_cubes
            );
        }
        Err(err) => println!("Error: {:?}", err),
    }
}
