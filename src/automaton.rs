use std::vec::Vec;
use std::mem::swap;
use rand::Rng;


pub struct Automaton {
    pub m: usize,
    pub n: usize,
    pub q: u32,
    pub cells: Vec<Vec<u32>>,
    pub temp: Vec<Vec<u32>>,
}


pub trait Rules {
    fn next_state(&self, i: usize, j: usize) -> u32;
    fn next(&mut self);
}

pub trait Init {
    fn init_state(&mut self, s: u32);
    fn init_rand(&mut self);
}


impl Automaton {
    pub fn new(m: usize, n: usize, q:u32, cells: Vec<Vec<u32>>) -> Self {
        Automaton {
            m,
            n,
            q,
            cells: cells.clone(),
            temp: cells
        }
    }

    pub fn init_state(&mut self, s: u32) {
        for i in 1..self.m - 1 {
            for j in 1..self.n - 1 {
                self.cells[i][j] = s;
                self.temp[i][j] = s;
            }
        }
    }

    pub fn init_rand(&mut self) {
        for i in 1..self.m - 1 {
            for j in 1..self.n - 1 {
                let s = rand::thread_rng().gen_range(0..self.q);
                self.cells[i][j] = s;
                self.temp[i][j] = s;
            }
        }
    }

    pub fn swap_buffer(&mut self) {
        swap(&mut self.cells, &mut self.temp);
    }

    pub fn get_neighbours(&self, i: usize, j: usize) -> Vec<u32> {
        let neighbours = vec![
            self.cells[i-1][j-1],
            self.cells[i-1][j],
            self.cells[i-1][j+1],
            self.cells[i][j-1],
            self.cells[i][j+1],
            self.cells[i+1][j-1],
            self.cells[i+1][j],
            self.cells[i+1][j+1],
        ];
        neighbours
    }
}
