extern crate cgol;

use cgol::{GameOfLife, GameState, GameError, GameOfLifeSettings};

#[test]
fn test_create_game() {

    /// Nathan wants to study Conway's Game of Life. Specifically, he is interested in a special
    /// class of patterns called (spaceships)[https://en.wikipedia.org/wiki/Spaceship_(cellular_automaton)].
    /// He starts by defining the initial parameters of the simulation:
    ///  - the world should consist of a 50 x 50 array of dead cells
    ///  - except for a handful of carefully selected cells which will form the initial population.
    let settings = GameOfLifeSettings::new()
        .set_dimensions(50, 50)
        .set_live_cell(1, 0)
        .set_live_cell(2, 1)
        .set_live_cell(0, 2)
        .set_live_cell(1, 2)
        .set_live_cell(2, 2);

    /// The initial parameters are passed to the simulation object at instantiation.
    let mut gol = GameOfLife::new(settings);

    /// Nathan can verify this is in fact a fresh Game instance by checking the number of iterations
    /// that have already been run.
    assert_eq!(gol.current_iteration(), 0);

    /// Calling update on the GameOfLife object causes the simulation to run exactly 1 iteration of
    /// the Game of Life. The state of the Game changes according to the (rules)[https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life].
    gol.update();

    /// The current state of the simulation is easily accessible. Nathan can check whether individual
    /// cells are currently alive or dead, allowing him to study the emergent behavior of the
    /// spaceship pattern.
    let current_state = gol.get_state();
    assert!(current_state.get_cell_state(0, 1));
    assert!(current_state.get_cell_state(2, 1));
    assert!(current_state.get_cell_state(1, 2));
    assert!(current_state.get_cell_state(2, 2));
    assert!(current_state.get_cell_state(1, 3));
}

