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
            let dimensions_str: &str = lines.get(0).unwrap();
            let x = usize::from_str(dimensions_str)?;

            let mut state: Vec<Vec<u8>> = Vec::new();

            for i in 1..=x {
                let line_opt = lines.get(i);
                match line_opt {
                    Some(row) => {
                        let line: Vec<&str> = row.split(',').collect();
                        let line: Vec<Result<u8, std::num::ParseIntError>> = line.iter()
                                                                                 .map(|s| u8::from_str(s))
                                                                                 .collect();

//                        state.push(line);
                    },
                    None => return Err(GameError::InvalidStateFile),
                }
            }

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
