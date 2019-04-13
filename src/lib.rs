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

        // Rules:
        // 1. Any live cell with fewer than 2 neighbors dies, as if by underpopulation
        // 2. Any live cell with 2 or 3 live neighbors lives on to the next generation
        // 3. Any live cell with more than 3 neighbors dies, as if by overpopulation
        // 4. Any dead cell with exactly 3 live neighbors becomes a live cell, as if by reproduction

        let mut next_state_opt: Option<GameState> = None;

        if let Some(state) = &self.game_state {
            self.iterations += 1;

            let world_dimensions = state.get_dimensions();
            let mut next_state = GameState{
                state: vec![vec![false; world_dimensions.1]; world_dimensions.0]
            };

            for i in 0..state.state.len() {
                for j in 0..state.state[i].len() {
                    let live_neighbors = self.get_number_of_live_neighbors(i, j);

                    if state.get_cell_state(i, j) {
                        if live_neighbors < 2 || live_neighbors > 3 {
                            next_state.set_cell_state(i, j, false);
                        }
                    } else if live_neighbors == 3 {
                        next_state.set_cell_state(i, j, true);
                    }
                }
            }

            next_state_opt = Some(next_state);
        }

        self.game_state = next_state_opt;
    }

    fn get_number_of_live_neighbors(&self, x: usize, y: usize) -> i32 {
        let mut live_neighbors = 0;

        if let Some(game_state) = &self.game_state {
            let mut neighbors = Vec::new();

            let world_dimensions = game_state.get_dimensions();
            if x == 0 && y == 0 {
                neighbors = vec![
                    game_state.get_cell_state(x + 1, y),
                    game_state.get_cell_state(x, y + 1),
                    game_state.get_cell_state(x + 1, y + 1)
                ];
            }

            if x > 0 && x < (world_dimensions.0 - 1) && y == 0 {
                neighbors = vec![
                    game_state.get_cell_state(x - 1, y),
                    game_state.get_cell_state(x + 1, y),
                    game_state.get_cell_state(x - 1, y + 1),
                    game_state.get_cell_state(x, y + 1),
                    game_state.get_cell_state(x + 1, y + 1)
                ];
            }

            if x == (world_dimensions.0 - 1) && y == 0 {
                neighbors = vec![
                    game_state.get_cell_state(x - 1, y),
                    game_state.get_cell_state(x - 1, y + 1),
                    game_state.get_cell_state(x, y + 1),
                ];
            }

            if x == 0 && y > 0 && y < (world_dimensions.1 - 1) {
                neighbors = vec![
                    game_state.get_cell_state(x, y - 1),
                    game_state.get_cell_state(x + 1, y - 1),
                    game_state.get_cell_state(x + 1, y),
                    game_state.get_cell_state(x, y + 1),
                    game_state.get_cell_state(x + 1, y + 1)
                ];
            }

            if x > 0 && y > 0 && x < (world_dimensions.0 - 1) && y < (world_dimensions.1 - 1) {
                neighbors = vec![
                    game_state.get_cell_state(x - 1, y - 1),
                    game_state.get_cell_state(x, y - 1),
                    game_state.get_cell_state(x + 1, y - 1),
                    game_state.get_cell_state(x - 1, y),
                    game_state.get_cell_state(x + 1, y),
                    game_state.get_cell_state(x - 1, y + 1),
                    game_state.get_cell_state(x, y + 1),
                    game_state.get_cell_state(x + 1, y + 1)
                ];
            }

            if x == (world_dimensions.0 - 1) && y > 0 && y < (world_dimensions.1 - 1) {
                neighbors = vec![
                    game_state.get_cell_state(x - 1, y - 1),
                    game_state.get_cell_state(x, y - 1),
                    game_state.get_cell_state(x - 1, y),
                    game_state.get_cell_state(x - 1, y + 1),
                    game_state.get_cell_state(x, y + 1),
                ];
            }

            if x == 0 && y == (world_dimensions.1 - 1) {
                neighbors = vec![
                    game_state.get_cell_state(x, y - 1),
                    game_state.get_cell_state(x + 1, y - 1),
                    game_state.get_cell_state(x + 1, y)
                ];
            }

            if x > 0 && x < (world_dimensions.0 - 1) && y == (world_dimensions.1 - 1) {
                neighbors = vec![
                    game_state.get_cell_state(x - 1, y - 1),
                    game_state.get_cell_state(x, y - 1),
                    game_state.get_cell_state(x + 1, y - 1),
                    game_state.get_cell_state(x - 1, y),
                    game_state.get_cell_state(x + 1, y)
                ];
            }

            if x == (world_dimensions.0 - 1) && y == (world_dimensions.1 - 1) {
                neighbors = vec![
                    game_state.get_cell_state(x - 1, y - 1),
                    game_state.get_cell_state(x, y - 1),
                    game_state.get_cell_state(x - 1, y)
                ];
            }

            for neighbor in neighbors {
                if neighbor {
                    live_neighbors += 1;
                }
            }
        }

        return live_neighbors;
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
    pub fn set_cell_state(&mut self, x: usize, y: usize, state: bool) -> Result<(), GameError> {
        if self.coord_in_world(x, y) {
            self.state[x][y] = state;
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

    pub fn get_cell_state(&self, x: usize, y: usize) -> bool {
        self.state[x][y]
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
        let lines: Vec<&str> = contents.split('\n').collect();

        if lines.len() > 0 {
            let (x, y) = GameStateBuilder::parse_coordinate(lines[0])?;
            let state: Vec<Vec<bool>> = vec![vec![false; y as usize]; x as usize];
            let mut game_state = GameState { state };

            for i in 1..lines.len() {
                let (live_cell_x, live_cell_y) = GameStateBuilder::parse_coordinate(lines[i])?;
                game_state.set_cell_state(live_cell_x, live_cell_y, true)?;
            }

            return Ok(game_state);
        }

        Err(GameError::InvalidStateFile(String::from("Empty state file")))
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
        if let Err(game_error) = gol.set_state("resources/test_states/single_live_cell.state") {
            panic!("resources/test_states/single_live_cell.state should be a valid state");
        }
        {
            let state = gol.get_state().unwrap();
            assert!(state.get_cell_state(1, 1));
        }
        gol.update();
        {
            let state = gol.get_state().unwrap();
            assert!(!state.get_cell_state(1, 1));
        }
    }

    #[test]
    fn test_cell_with_two_neighbors_lives() {
        let mut gol = GameOfLife::new();

        if let Err(game_error) = gol.set_state("resources/test_states/cell_with_two_neighbors.state") {
            panic!("resources/test_states/cell_with_two_neighbors.state should be a valid state");
        }
        {
            let state = gol.get_state().unwrap();
            assert!(state.get_cell_state(0,0));
            assert!(state.get_cell_state(1,0));
            assert!(state.get_cell_state(1,1));
        }
        gol.update();
        {
            let state = gol.get_state().unwrap();
            assert!(!state.get_cell_state(0,0));
            assert!(!state.get_cell_state(1,0));
            assert!(state.get_cell_state(1,1));
        }
    }
}
