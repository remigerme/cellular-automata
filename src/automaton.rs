use std::vec::Vec;
use std::mem::swap;
use rand::Rng;


pub struct Automaton {
    pub m: usize,
    pub n: usize,
    pub q: u32,
    pub torus: bool,
    colors: Vec<u32>,
    pub cells: Vec<Vec<u32>>,
    pub temp: Vec<Vec<u32>>,
}


pub trait Access {
    fn get_size(&self) -> (usize, usize);
    fn get_cells(&self) -> Vec<Vec<u32>>;
    fn get_cell(&self, i: usize, j: usize) -> u32;
    fn get_cell_color(&self, i: usize, j: usize) -> u32;
}


pub trait Init {
    fn init_state(&mut self, s: u32);
    fn init_rand(&mut self);
}


pub trait Rules {
    fn next_state(&self, i: usize, j: usize) -> u32;
    fn next(&mut self);
}


impl Automaton {
    pub fn new(m: usize, n: usize, q:u32, torus: bool, colors: Vec<u32>, cells: Vec<Vec<u32>>) -> Self {
        Automaton {
            m,
            n,
            q,
            torus,
            colors,
            cells: cells.clone(),
            temp: cells
        }
    }

    pub fn swap_buffer(&mut self) {
        swap(&mut self.cells, &mut self.temp);
    }

    pub fn get_neighbours(&self, i: usize, j: usize) -> Vec<u32> {
        let m = self.m;
        let n = self.n;
        let neighbours = vec![
            self.cells[(i-1) % m][(j-1) % n],
            self.cells[(i-1) % m][j],
            self.cells[(i-1) % m][(j+1) % n],
            self.cells[i][(j-1) % n],
            self.cells[i][(j+1) % n],
            self.cells[(i+1) % m][(j-1) % n],
            self.cells[(i+1) % m][j],
            self.cells[(i+1) % m][(j+1) % n],
        ];
        neighbours
    }
}


impl Access for Automaton {
    fn get_size(&self) -> (usize, usize) {
       (self.m, self.n) 
    }

    fn get_cells(&self) -> Vec<Vec<u32>> {
        self.cells.clone()
    }

    fn get_cell(&self, i: usize, j: usize) -> u32 {
        self.cells[i][j]
    }

    fn get_cell_color(&self, i: usize, j: usize) -> u32 {
        self.colors[self.cells[i][j] as usize]
    }
}


impl Init for Automaton {
    fn init_state(&mut self, s: u32) {
        let (i_min, i_max) = if self.torus {(0, self.m)} else {(1, self.m - 1)};
        let (j_min, j_max) = if self.torus {(0, self.n)} else {(1, self.n - 1)};
        for i in i_min..i_max {
            for j in j_min..j_max {
                self.cells[i][j] = s;
                self.temp[i][j] = s;
            }
        }
    }

    fn init_rand(&mut self) {
        let (i_min, i_max) = if self.torus {(0, self.m)} else {(1, self.m - 1)};
        let (j_min, j_max) = if self.torus {(0, self.n)} else {(1, self.n - 1)};
        for i in i_min..i_max {
            for j in j_min..j_max {
                let s = rand::thread_rng().gen_range(0..self.q);
                self.cells[i][j] = s;
                self.temp[i][j] = s;
            }
        }
    }
}
