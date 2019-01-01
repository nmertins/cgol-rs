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
    fn from_file(file_path: &str) -> Result<GameState, GameError> {
        let contents = fs::read_to_string(file_path)?;

        let lines: Vec<&str> = contents.split('\n').collect();
        if lines.len() > 1 {
            let dimensions_str: Vec<&str> = lines.get(0).unwrap().split(',').collect();
            let dimensions_num: (u32, u32) = (
                u32::from_str(dimensions_str[0])?,
                u32::from_str(dimensions_str[1])?
            );

            return Ok(GameState{});
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
        let valid_state = GameState::from_file("resources/valid_test.state");
        assert!(valid_state.is_ok());

        let invalid_state = GameState::from_file("resources/empty_file.state");
        assert!(invalid_state.is_err());
    }
}
