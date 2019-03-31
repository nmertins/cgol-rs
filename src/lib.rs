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
    state: Vec<(u8, u8)>,
}

struct GameStateBuilder;

impl GameStateBuilder {
    fn from_file(file_path: &str) -> Result<GameState, GameError> {
        let contents = fs::read_to_string(file_path)?;
        let lines: Vec<&str> = contents.split('\n').collect();

        let y = lines.len();
        if lines.len() > 0 {
            let mut state: Vec<(u8, u8)> = Vec::new();

            let (x, y) = GameStateBuilder::parse_coordinate(lines[0])?;

            for i in 1..lines.len() {
                let (live_cell_x, live_cell_y) = GameStateBuilder::parse_coordinate(lines[i])?;
                if live_cell_x < 0 || live_cell_y < 0 || live_cell_x >= x || live_cell_y >= y {
                    return Err(GameError::InvalidStateFile(String::from(format!("Cell outside World: ({}, {})", live_cell_x, live_cell_y))));
                }
                
                state.push((x, y));
            }

            // for line in lines {
            //     let values_str: Vec<&str> = line.split(',').collect();
            //     let x = values_str.len();
            //     if x != y {
            //         return Err(GameError::InvalidStateFile(String::from("Dimensions not square.")))
            //     }
            //     let u8_from_str_results: Vec<Result<u8, std::num::ParseIntError>> = values_str.iter()
            //                                                                                   .map(|s| u8::from_str(s))
            //                                                                                   .collect();
            //     let mut values_u8 = Vec::new();
            //     for result in u8_from_str_results {
            //         let value = result?;
            //         values_u8.push(value);
            //     }
            //     state.push(values_u8);
            // }

            return Ok(GameState{state});
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

    macro_rules! assert_dead {
        ($state_file:ident, $coord_tuple:ident) => (

        )
    }

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
                assert_eq!(valid_state.state.len(), 3);
                match valid_state.state.get(0) {
                    Some(row) => {
                        assert_eq!(row.len(), 3);
                        assert_eq!(Some(&1), row.get(0));
                    },
                    None => assert!(false, "state field for valid_state is None. Expected a 3x3 state.")
                }
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
        let dead_cell = (1,1);
        assert_dead!(state, dead_cell);
    }
}
