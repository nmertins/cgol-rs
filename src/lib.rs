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
    InvalidStateFile
}

impl std::convert::From<io::Error> for GameError {
    fn from(_: io::Error) -> GameError {
        GameError::InvalidStateFile
    }
}

impl std::convert::From<std::num::ParseIntError> for GameError {
    fn from(_: std::num::ParseIntError) -> GameError {
        GameError::InvalidStateFile
    }
}

impl GameState {
    pub fn from_file(file_path: &str) -> Result<GameState, GameError> {
        let contents = fs::read_to_string(file_path)?;
        let lines: Vec<&str> = contents.split('\n').collect();

        if lines.len() > 1 {
            let dimensions_str: Vec<&str> = lines.get(0).unwrap().split(',').collect();
            let x = usize::from_str(dimensions_str[0])?;

            return Ok(GameState{state: vec![vec![0u8; x]; x]});
        }

        Err(GameError::InvalidStateFile)
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
                    None => assert!(false, "Shouldn't ger here")
                }
            },
            Err(error) => assert!(false, "Shouldn't get here")
        }

        let invalid_state_result = GameState::from_file("resources/invalid_dimensions.state");
        assert!(invalid_state_result.is_err());

        let empty_state_result = GameState::from_file("resources/empty_file.state");
        assert!(empty_state_result.is_err());
    }
}
