use std::fs;
use std::io;
use std::str::FromStr;

pub struct GameOfLife {
    iterations: u32,
    game_state: Option<GameState>,
}

impl GameOfLife {
    pub fn new() -> GameOfLife {
        GameOfLife {
            iterations: 0,
            game_state: None,
        }
    }

    pub fn current_iteration(&self) -> u32 {
        self.iterations
    }

    pub fn set_state(&mut self, file_path: &str) -> Result<(), GameError> {
        let state = GameStateBuilder::from_file(file_path)?;
        self.game_state = Some(state);
        Result::Ok(())
    }

    pub fn get_state(&self) -> Result<&GameState, GameError> {
        match &self.game_state {
            Some(state) => return Ok(state),
            None => return Err(GameError::EmptyGameState)
        }
    }

    pub fn update(&mut self) {
        self.iterations += 1;
    }
}

pub enum GameError {
    InvalidStateFile(String),
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
            GameError::EmptyGameState => write!(f, "EmptyGameState error")
        }
    }
}

pub struct GameState {
    state: Vec<Vec<bool>>,
}

impl GameState {
    pub fn set_live(&mut self, x: u8, y: u8) -> Result<(), GameError> {
        if self.coord_in_world(x, y) {
            self.state[x as usize][y as usize] = true;
        }
        else {
            // return Err(GameError::InvalidStateFile(String::from(format!("Cell outside World: ({}, {})", x, y))));
            // just ignore cells outside the world
        }

        return Ok(());
    }

    pub fn get_dimensions(&self) -> (usize, usize) {
        let world_x = self.state[0].len();
        let world_y = self.state.len();

        (world_x, world_y)
    }

    pub fn get_cell_state(&self, x: u8, y: u8) -> bool {
        self.state[x as usize][y as usize]
    }

    fn coord_in_world(&self, x: u8, y: u8) -> bool {
        let world_x = self.state[0].len();
        let world_y = self.state.len();

        return x >= world_x as u8 || y >= world_y as u8;
    }
}

struct GameStateBuilder;

impl GameStateBuilder {
    fn from_file(file_path: &str) -> Result<GameState, GameError> {
        let contents = fs::read_to_string(file_path)?;
        let lines: Vec<&str> = contents.split('\n').collect();

        let y = lines.len();
        if lines.len() > 0 {
            let (x, y) = GameStateBuilder::parse_coordinate(lines[0])?;
            let state: Vec<Vec<bool>> = vec![vec![false; y as usize]; x as usize];
            let mut game_state = GameState { state };

            for i in 1..lines.len() {
                let (live_cell_x, live_cell_y) = GameStateBuilder::parse_coordinate(lines[i])?;
                game_state.set_live(live_cell_x, live_cell_y)?;
            }

            return Ok(game_state);
        }

        Err(GameError::InvalidStateFile(String::from("Empty state file")))
    }

    fn parse_coordinate(coord_str: &str) -> Result<(u8, u8), GameError> {
        let dimensions: Vec<&str> = coord_str.split(',').collect();
        if dimensions.len() != 2 { return Err(GameError::InvalidStateFile(String::from("Failed to parse dimensions."))) }
        let x = u8::from_str(dimensions[0])?;
        let y = u8::from_str(dimensions[1])?;

        Ok((x, y))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_increments_iterations() {
        let mut gol = GameOfLife::new();
        assert_eq!(gol.iterations, 0);

        gol.update();
        assert_eq!(gol.iterations, 1);
    }

    #[test]
    fn test_state_file_format() {
        let valid_state_result = GameStateBuilder::from_file("resources/test_states/valid_test.state");
        match valid_state_result {
            Ok(valid_state) => {
                let (x, y) = valid_state.get_dimensions();
                assert_eq!(x, 3);
                assert_eq!(y, 3);
                assert!(valid_state.get_cell_state(1, 1));
            },
            Err(error) => {
                match error {
                    GameError::InvalidStateFile(message) => assert!(false, format!("Error reading state file resources/valid_test.state: {}", message)),
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
        let mut gol = GameOfLife::new();
        gol.set_state("resources/test/single_live_cell.state");
        gol.update();
        let state = gol.get_state().unwrap();
        assert!(!state.get_cell_state(1, 1));
    }
}
