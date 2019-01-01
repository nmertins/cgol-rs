use std::fs::File;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};

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

impl GameState {
    fn from_file(file_path: &str) -> std::io::Result<GameState> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let lines :Vec<&str> = contents.split('\n').collect();

        if lines.len() == 2 {
            return Ok(GameState{});
        }


        Err(Error::new(ErrorKind::InvalidInput, "Invalid state file format"))
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
