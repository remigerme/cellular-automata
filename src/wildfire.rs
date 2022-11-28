use rand::Rng;
use crate::automaton::{Automaton, Access, Init, Rules};


const ROCK: u32 = 0;
const GREY: u32 = 0xB0B0B0;
const ASHES: u32 = 1;
const BLACK: u32 = 0x3B3737;
const YOUNG: u32 = 2;
const GREEN: u32 = 0x68E180;
const OLD: u32 = 3;
const DARK_GREEN: u32 = 0x24A609;
const NEW_FIRE: u32 = 4;
const YELLOW: u32 = 0xF2F26E;
const FIRE: u32 = 5;
const ORANGE: u32 = 0xE48F01;
const FADING_FIRE: u32 = 6;
const RED: u32 = 0xBA2E0B;


pub struct WildfireAutomaton(Automaton);


impl WildfireAutomaton {
    pub fn new(m: usize, n: usize, torus: bool, cells: Vec<Vec<u32>>) -> Self {
        let q = 7;
        let colors = vec![GREY, BLACK, GREEN, DARK_GREEN, YELLOW, ORANGE, RED];
        WildfireAutomaton(Automaton::new(m, n, q, torus, colors, cells))
    }
}


impl Rules for WildfireAutomaton {
    fn next_state(&self, i: usize, j: usize) -> u32 {
        let mut rng = rand::thread_rng();
        match self.0.cells[i][j] {
            ROCK => ROCK,
            ASHES => if rng.gen::<f64>() < 0.001 {YOUNG} else {ASHES},
            YOUNG => {
                let neighbours = self.0.get_neighbours(i, j);
                if rng.gen::<f64>() < 0.01 && neighbours.contains(&NEW_FIRE) {
                    NEW_FIRE
                } else if rng.gen::<f64>() < 0.02 && neighbours.contains(&FIRE) {
                    NEW_FIRE
                } else if rng.gen::<f64>() < 0.01 && neighbours.contains(&FADING_FIRE) {
                    NEW_FIRE
                } else if rng.gen::<f64>() < 0.005 {
                    OLD
                } else {
                    YOUNG
                }
            },
            OLD => {
                let neighbours = self.0.get_neighbours(i, j);
                if rng.gen::<f64>() < 0.1 && neighbours.contains(&NEW_FIRE) {
                    NEW_FIRE
                } else if rng.gen::<f64>() < 0.2 && neighbours.contains(&FIRE) {
                    NEW_FIRE
                } else if rng.gen::<f64>() < 0.1 && neighbours.contains(&FADING_FIRE) {
                    NEW_FIRE
                } else if rng.gen::<f64>() < 0.00005 && neighbours.iter().filter(|&n| *n == OLD).count() >= 5 {
                    NEW_FIRE
                } else {
                    OLD
                }
            },
            NEW_FIRE => if rng.gen::<f64>() < 0.1 {FIRE} else {NEW_FIRE},
            FIRE => if rng.gen::<f64>() < 0.1 {FADING_FIRE} else {FIRE},
            FADING_FIRE => if rng.gen::<f64>() < 0.1 {ASHES} else {FADING_FIRE},
            _ => ROCK,
        }
    }

    fn next(&mut self) {
        let (i_min, i_max) = if self.0.torus {(0, self.0.m)} else {(1, self.0.m - 1)};
        let (j_min, j_max) = if self.0.torus {(0, self.0.n)} else {(1, self.0.n - 1)};
        for i in i_min..i_max {
            for j in j_min..j_max {
                self.0.temp[i][j] = self.next_state(i, j);
            }
        }
        self.0.swap_buffer();
    }
}


// The end of the file is used to delegate functions
impl Access for WildfireAutomaton {
    fn get_size(&self) -> (usize, usize) {
        self.0.get_size()
    }

    fn get_cells(&self) -> Vec<Vec<u32>> {
        self.0.get_cells()
    }

    fn get_cell(&self, i: usize, j: usize) -> u32 {
        self.0.get_cell(i, j)
    }

    fn get_cell_color(&self, i: usize, j: usize) -> u32 {
        self.0.get_cell_color(i, j)
    }
}


impl Init for WildfireAutomaton {
    fn init_rand(&mut self) {
        self.0.init_rand()
    }

    fn init_state(&mut self, s: u32) {
        self.0.init_state(s)
    }
}
