extern crate csv;

use std::io::Error;
use csv::{Reader, ReaderBuilder};

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
    fn from_file<'a>(file_path: &str) -> Result<GameState, &'a str> {
        let rdr_result = ReaderBuilder::new().from_path(file_path);
        if let Ok(mut rdr) = rdr_result {
            for record_result in rdr.records() {
                if let Ok(record) = record_result {
                    println!("{:?}", record);
                }
            }
        }

        Err("Error parsing .state file.")
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
