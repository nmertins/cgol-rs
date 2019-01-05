use std::fs;
use std::io;
use std::str::FromStr;

struct GameOfLife {
    iterations: u32,
}

impl GameOfLife {
    pub fn new() -> GameOfLife {
        GameOfLife {
            iterations: 0
        }
    }

    fn update(&mut self) {
        self.iterations += 1;
    }
}

struct GameState {
    state: Vec<Vec<u8>>,
}

enum GameError {
    InvalidStateFile(String)
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

impl GameState {
    pub fn from_file(file_path: &str) -> Result<GameState, GameError> {
        let contents = fs::read_to_string(file_path)?;
        let mut lines: Vec<&str> = contents.split('\n').collect();

        if lines.len() > 1 {
            let dimensions_str: &str = lines.pop().unwrap();
            let x = usize::from_str(dimensions_str)?;

            let mut state: Vec<Vec<u8>> = Vec::new();

            for line in lines {
                let values_str: Vec<&str> = line.split(',').collect();
                let str_to_u8_results: Vec<Result<u8, std::num::ParseIntError>> = values_str.iter().map(|s| u8::from_str(s)).collect();
                let mut values_u8 = Vec::new();
                for result in str_to_u8_results {
                    let value = result?;
                    values_u8.push(value);
                }
                state.push(values_u8);
            }

            return Ok(GameState{state});
        }

        Err(GameError::InvalidStateFile(String::from("Empty state file")))
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
        let valid_state_result = GameState::from_file("resources/valid_test.state");
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
                    GameError::InvalidStateFile(message) => assert!(false, format!("Error reading state file resources/valid_test.state: {}", message))
                }

            }
        }

        let invalid_state_result = GameState::from_file("resources/invalid_dimensions.state");
        assert!(invalid_state_result.is_err());

        let empty_state_result = GameState::from_file("resources/empty_file.state");
        assert!(empty_state_result.is_err());
    }
}
