use std::fs;
use std::io;
use std::str::FromStr;

pub struct GameOfLifeSettings {
    initial_state: GameState
}

impl GameOfLifeSettings {
    pub fn new() -> GameOfLifeSettings {
        let initial_state = GameState {
            state: vec![vec![false, false, false],
                        vec![false, false, false],
                        vec![false, false, false]]
        };

        GameOfLifeSettings {
            initial_state
        }
    }

    pub fn from_file(file_path: &str) -> Result<GameOfLifeSettings, GameError> {
        let initial_state = GameStateBuilder::from_file(file_path)?;
        let settings = GameOfLifeSettings {
            initial_state
        };

        Ok(settings)
    }

    pub fn get_dimensions(&self) -> (usize, usize) {
        self.initial_state.get_dimensions()
    }

    pub fn set_dimensions(mut self, x: usize, y: usize) -> Self {
        self.initial_state = GameState {
            state: vec![vec![false; x]; y]
        };

        self
    }

    pub fn set_live_cell(mut self, x: usize, y: usize) -> Self {
        match self.initial_state.set_cell_state(x, y, true) {
            Ok(_) => {},
            Err(err) => println!("{:?}", err),
        }

        self
    }
}

pub struct GameOfLife {
    iterations: u32,
    game_state: GameState,
}

impl GameOfLife {
    pub fn new(settings: GameOfLifeSettings) -> GameOfLife {
        GameOfLife {
            iterations: 0,
            game_state: settings.initial_state,
        }
    }

    pub fn current_iteration(&self) -> u32 {
        self.iterations
    }

    pub fn get_state(&self) -> &GameState {
        &self.game_state
    }

    pub fn update(&mut self) {
        self.iterations += 1;

        // Rules:
        // 1. Any live cell with fewer than 2 neighbors dies, as if by underpopulation
        // 2. Any live cell with 2 or 3 live neighbors lives on to the next generation
        // 3. Any live cell with more than 3 neighbors dies, as if by overpopulation
        // 4. Any dead cell with exactly 3 live neighbors becomes a live cell, as if by reproduction

        let world_dimensions = self.game_state.get_dimensions();
        let mut next_state = GameState{
            state: vec![vec![false; world_dimensions.0]; world_dimensions.1]
        };

        for y in 0..self.game_state.state.len() {
            for x in 0..self.game_state.state[y].len() {
                let live_neighbors = self.get_number_of_live_neighbors(x, y);

                if self.game_state.get_cell_state(x, y) {
                    if live_neighbors < 2 || live_neighbors > 3 {
                        next_state.set_cell_state(x, y, false);
                    } else {
                        next_state.set_cell_state(x, y, true);
                    }
                } else if live_neighbors == 3 {
                    next_state.set_cell_state(x, y, true);
                }
            }
        }

        self.game_state = next_state;
    }

    fn get_number_of_live_neighbors(&self, x: usize, y: usize) -> i32 {
        let mut live_neighbors = 0;

        let mut neighbors = Vec::new();

        let world_dimensions = self.game_state.get_dimensions();
        if x == 0 && y == 0 {
            neighbors = vec![
                self.game_state.get_cell_state(x + 1, y),
                self.game_state.get_cell_state(x, y + 1),
                self.game_state.get_cell_state(x + 1, y + 1)
            ];
        }

        if x > 0 && x < (world_dimensions.0 - 1) && y == 0 {
            neighbors = vec![
                self.game_state.get_cell_state(x - 1, y),
                self.game_state.get_cell_state(x + 1, y),
                self.game_state.get_cell_state(x - 1, y + 1),
                self.game_state.get_cell_state(x, y + 1),
                self.game_state.get_cell_state(x + 1, y + 1)
            ];
        }

        if x == (world_dimensions.0 - 1) && y == 0 {
            neighbors = vec![
                self.game_state.get_cell_state(x - 1, y),
                self.game_state.get_cell_state(x - 1, y + 1),
                self.game_state.get_cell_state(x, y + 1),
            ];
        }

        if x == 0 && y > 0 && y < (world_dimensions.1 - 1) {
            neighbors = vec![
                self.game_state.get_cell_state(x, y - 1),
                self.game_state.get_cell_state(x + 1, y - 1),
                self.game_state.get_cell_state(x + 1, y),
                self.game_state.get_cell_state(x, y + 1),
                self.game_state.get_cell_state(x + 1, y + 1)
            ];
        }

        if x > 0 && y > 0 && x < (world_dimensions.0 - 1) && y < (world_dimensions.1 - 1) {
            neighbors = vec![
                self.game_state.get_cell_state(x - 1, y - 1),
                self.game_state.get_cell_state(x, y - 1),
                self.game_state.get_cell_state(x + 1, y - 1),
                self.game_state.get_cell_state(x - 1, y),
                self.game_state.get_cell_state(x + 1, y),
                self.game_state.get_cell_state(x - 1, y + 1),
                self.game_state.get_cell_state(x, y + 1),
                self.game_state.get_cell_state(x + 1, y + 1)
            ];
        }

        if x == (world_dimensions.0 - 1) && y > 0 && y < (world_dimensions.1 - 1) {
            neighbors = vec![
                self.game_state.get_cell_state(x - 1, y - 1),
                self.game_state.get_cell_state(x, y - 1),
                self.game_state.get_cell_state(x - 1, y),
                self.game_state.get_cell_state(x - 1, y + 1),
                self.game_state.get_cell_state(x, y + 1),
            ];
        }

        if x == 0 && y == (world_dimensions.1 - 1) {
            neighbors = vec![
                self.game_state.get_cell_state(x, y - 1),
                self.game_state.get_cell_state(x + 1, y - 1),
                self.game_state.get_cell_state(x + 1, y)
            ];
        }

        if x > 0 && x < (world_dimensions.0 - 1) && y == (world_dimensions.1 - 1) {
            neighbors = vec![
                self.game_state.get_cell_state(x - 1, y - 1),
                self.game_state.get_cell_state(x, y - 1),
                self.game_state.get_cell_state(x + 1, y - 1),
                self.game_state.get_cell_state(x - 1, y),
                self.game_state.get_cell_state(x + 1, y)
            ];
        }

        if x == (world_dimensions.0 - 1) && y == (world_dimensions.1 - 1) {
            neighbors = vec![
                self.game_state.get_cell_state(x - 1, y - 1),
                self.game_state.get_cell_state(x, y - 1),
                self.game_state.get_cell_state(x - 1, y)
            ];
        }

        for neighbor in neighbors {
            if neighbor {
                live_neighbors += 1;
            }
        }

        return live_neighbors;
    }
}

pub enum GameError {
    InvalidStateFile(String),
    InvalidWorldCoordinates(String),
    EmptyGameState
}

impl std::convert::From<io::Error> for GameError {
    fn from(e: io::Error) -> GameError {
        GameError::InvalidStateFile(e.to_string())
    }
}

impl std::convert::From<std::num::ParseIntError> for GameError {
    fn from(e: std::num::ParseIntError) -> GameError {
        GameError::InvalidStateFile(e.to_string())
    }
}

impl std::fmt::Debug for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GameError::InvalidStateFile(message) => write!(f, "InvalidStateFile error: {}", message),
            GameError::InvalidWorldCoordinates(message) => write!(f, "InvalidWorldCoordinates error: {}", message),
            GameError::EmptyGameState => write!(f, "EmptyGameState error")
        }
    }
}

pub struct GameState {
    state: Vec<Vec<bool>>,
}

impl GameState {
    pub fn set_cell_state(&mut self, x: usize, y: usize, state: bool) -> Result<(), GameError> {
        if self.coord_in_world(x, y) {
            self.state[y][x] = state;
        }
        else {
            return Err(GameError::InvalidWorldCoordinates(String::from(format!("Cell outside World: ({}, {})", x, y))));
        }

        return Ok(());
    }

    pub fn get_dimensions(&self) -> (usize, usize) {
        let world_x = self.state[0].len();
        let world_y = self.state.len();

        (world_x, world_y)
    }

    pub fn get_cell_state(&self, x: usize, y: usize) -> bool {
        self.state[y][x]
    }

    fn coord_in_world(&self, x: usize, y: usize) -> bool {
        let world_x = self.state[0].len();
        let world_y = self.state.len();

        return x < world_x && y < world_y;
    }
}

struct GameStateBuilder;

impl GameStateBuilder {
    fn from_file(file_path: &str) -> Result<GameState, GameError> {
        let contents = fs::read_to_string(file_path)?;
        let mut lines = contents.lines();
        let mut game_state: Option<GameState> = None;

        let first_line = lines.next();
        if let Some(line) = first_line {
            let (x, y) = GameStateBuilder::parse_coordinate(line)?;
            let state: Vec<Vec<bool>> = vec![vec![false; x]; y];
            game_state = Some(GameState { state });
        }

        match game_state {
            None => { return Err(GameError::InvalidStateFile(String::from("Empty state file"))) },
            Some(mut game_state) => {
                for line in lines {
                    let (live_cell_x, live_cell_y) = GameStateBuilder::parse_coordinate(line)?;
                    game_state.set_cell_state(live_cell_x, live_cell_y, true)?;
                }

                Ok(game_state)
            },
        }
    }

    fn parse_coordinate(coord_str: &str) -> Result<(usize, usize), GameError> {
        let dimensions: Vec<&str> = coord_str.split(',').collect();
        if dimensions.len() != 2 { return Err(GameError::InvalidStateFile(String::from("Failed to parse dimensions."))) }
        let x = usize::from_str(dimensions[0])?;
        let y = usize::from_str(dimensions[1])?;

        Ok((x, y))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_increments_iterations() {
        let settings = GameOfLifeSettings::new();
        let mut gol = GameOfLife::new(settings);
        assert_eq!(gol.iterations, 0);

        gol.update();
        assert_eq!(gol.iterations, 1);
    }

    #[test]
    fn test_state_file_format() {
        let test_file_path = "resources/test_states/valid_test.state";
        let valid_state_result = GameStateBuilder::from_file(&test_file_path);
        match valid_state_result {
            Ok(valid_state) => {
                let size = valid_state.get_dimensions();
                assert_eq!(size, (3, 3));
                
                // live cells
                assert!(valid_state.get_cell_state(0, 0));

                // dead cells
                assert!(!valid_state.get_cell_state(1, 0));
                assert!(!valid_state.get_cell_state(2, 0));
                assert!(!valid_state.get_cell_state(0, 1));
                assert!(!valid_state.get_cell_state(1, 1));
                assert!(!valid_state.get_cell_state(2, 1));
                assert!(!valid_state.get_cell_state(0, 2));
                assert!(!valid_state.get_cell_state(1, 2));
                assert!(!valid_state.get_cell_state(2, 2));
            },
            Err(error) => {
                match error {
                    GameError::InvalidStateFile(message) => assert!(false, format!("Error reading state file {}: {}", test_file_path, message)),
                    _ => assert!(false, "Why is GameState::from_file returning an error besides InvalidStateFile error??")
                }

            }
        }

        let invalid_state_result = GameStateBuilder::from_file("resources/test_states/invalid_dimensions.state");
        assert!(invalid_state_result.is_err());

        let empty_state_result = GameStateBuilder::from_file("resources/test_states/empty_file.state");
        assert!(empty_state_result.is_err());

        let not_square_state_result = GameStateBuilder::from_file("resources/test_states/not_square.state");
        assert!(not_square_state_result.is_err());
    }

    #[test]
    fn test_single_live_cell_dies() {
        let settings_result = GameOfLifeSettings::from_file("resources/test_states/single_live_cell.state");
        if let Err(settings) = settings_result {
            panic!("resources/test_states/single_live_cell.state should be a valid state");
        }
        let mut gol = GameOfLife::new(settings_result.unwrap());
        {
            let state = gol.get_state();
            assert!(state.get_cell_state(1, 1));
        }
        gol.update();
        {
            let state = gol.get_state();
            assert!(!state.get_cell_state(1, 1));
        }
    }

    #[test]
    fn test_cell_with_two_neighbors_lives() {
        let settings_result = GameOfLifeSettings::from_file("resources/test_states/cell_with_two_neighbors.state");
        if let Err(settings) = settings_result {
            panic!("resources/test_states/cell_with_two_neighbors.state should be a valid state");
        }
        let mut gol = GameOfLife::new(settings_result.unwrap());
        {
            let state = gol.get_state();
            assert!(state.get_cell_state(0,0));
            assert!(state.get_cell_state(1,1));
            assert!(state.get_cell_state(2,2));
        }
        gol.update();
        {
            let state = gol.get_state();
            assert!(!state.get_cell_state(0,0));
            assert!(!state.get_cell_state(2,2));
            assert!(state.get_cell_state(1,1));
        }
    }

    #[test]
    fn test_setting_initial_game_state_programmatically() {
        let settings = GameOfLifeSettings::new();
        assert_eq!((3, 3), settings.get_dimensions());
        let settings = settings.set_dimensions(10, 10);
        assert_eq!((10, 10), settings.get_dimensions());

        let settings = settings.set_live_cell(5, 5);
        let gol = GameOfLife::new(settings);
        assert!(gol.get_state().get_cell_state(5, 5));
    }
}
