use crate::automaton::{Automaton, Rules, Draw};

pub struct GameOfLifeAutomaton(Automaton);


impl GameOfLifeAutomaton {
    pub fn new(m: usize, n: usize, cells: Vec<Vec<u32>>) -> Self {
        let q = 2;
        let colors = vec![0xFFFFFF, 0];
        GameOfLifeAutomaton(Automaton::new(m, n, q, colors, cells))
    }

    pub fn get_size(&self) -> (usize, usize) {
        (self.0.m, self.0.n)
    }

    pub fn get_cell(&self, i: usize, j: usize) -> u32 {
        self.0.cells[i][j]
    }

    pub fn get_cells(&self) -> Vec<Vec<u32>> {
        self.0.cells.clone()
    }

    pub fn init_state(&mut self, s: u32) {
        self.0.init_state(s);
    }

    pub fn init_rand(&mut self) {
        self.0.init_rand();
    }

    pub fn get_cell_color(&self, i: usize, j: usize) -> u32 {
        self.0.get_cell_color(i, j)
    }
}


impl Rules for GameOfLifeAutomaton {
    fn next_state(&self, i: usize, j: usize) -> u32 {
        let neighbours = self.0.get_neighbours(i, j);
        let nb_neighbours_alive = neighbours.iter().fold(0, |a, b| a + b);

        match self.0.cells[i][j] {
            0 => if nb_neighbours_alive == 3 {1} else {0},
            1 => if nb_neighbours_alive == 2 || nb_neighbours_alive == 3 {1} else {0},
            _ => 0
        }
    }

    fn next(&mut self) {
        for i in 1..self.0.m - 1 {
            for j in 1..self.0.n - 1 {
                self.0.temp[i][j] = self.next_state(i, j);
            }
        }
        self.0.swap_buffer();
    }
}
