#[cfg(test)]
mod tests {
    #[test]
    fn test_create_game() {
        /**
         * Nathan wants to study Conway's Game of Life. He starts by creating
         * an instance of the GameOfLife object. This type handles the state
         * and rules of a single Game of Life.
         */
        let gol = GameOfLife::new();

        /**
         * Nathan can verify this is in fact a fresh Game instance by checking the
         * number of iterations that have already been run.
         */
        assert_eq!(gol.iterations, 0);

        /**
         * Running the game isn't very exciting without seeding the initial
         * state. This is done with a special .state file.
         */
        gol.set_state("resources/gol_initial.state");

        /**
         * Calling update on the GameOfLife object causes the simulation to run
         * exactly 1 iteration of the Game of Life. The state of the Game changes
         * according to the (rules)[https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life].
         */
        gol.update();
    }
}
