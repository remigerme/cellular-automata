use std::vec::Vec;
use std::mem::swap;
use rand::Rng;


pub struct Automaton {
    m: usize,
    n: usize,
    q: u32,
    torus: bool,
    next_state: Box<dyn Fn(&Self, usize, usize) -> u32>,
    colors: Vec<u32>,
    cells: Vec<Vec<u32>>,
    temp: Vec<Vec<u32>>
}


impl Automaton {
    pub fn new(
        m: usize,
        n: usize,
        q: u32,
        torus: bool,
        next_state: Box<dyn Fn(&Automaton, usize, usize) -> u32>,
        colors: Vec<u32>,
        cells: Vec<Vec<u32>>
    ) -> Self {
        Automaton {
            m, n, q, torus, next_state, colors, cells: cells.clone(), temp: cells
        }
    }

    // Getters
    pub fn get_size(&self) -> (usize, usize) { (self.m, self.n) }
    pub fn get_q(&self) -> u32 { self.q }
    pub fn get_cells(&self) -> Vec<Vec<u32>> { self.cells.clone() }
    pub fn get_cell(&self, i: usize, j: usize) -> u32 {self.cells[i][j] }
    pub fn get_cell_color(&self, i: usize, j: usize) -> u32 { self.colors[self.cells[i][j] as usize] }

    pub fn get_von_neumann_neighbours(&self, i: usize, j: usize) -> Vec<u32> {
        let m = self.m;
        let n = self.n;
        vec![
            self.cells[(i - 1) % m][j],
            self.cells[i][(j + 1) % n],
            self.cells[(i + 1) % m][j],
            self.cells[i][(j - 1) % n]
        ]
    }

    pub fn get_moore_neighbours(&self, i: usize, j: usize) -> Vec<u32> {
        let m = self.m;
        let n = self.n;
        let mut neighbours = self.get_von_neumann_neighbours(i, j);
        neighbours.append(&mut vec![
            self.cells[(i - 1) % m][(j - 1) % n],
            self.cells[(i - 1) % m][(j + 1) % n],
            self.cells[(i + 1) % m][(j - 1) % n],
            self.cells[(i + 1) % m][(j + 1) % n],
        ]);
        neighbours
    }

    // Init
    pub fn init_state(&mut self, s: u32, edge: bool) {
        let (i_min, i_max) = if edge { (0, self.m )} else { (1, self.m - 1) };
        let (j_min, j_max) = if edge { (0, self.n) } else { (1, self.n - 1) };
        for i in i_min..i_max {
            for j in j_min..j_max {
                self.cells[i][j] = s;
            }
        }
    }

    pub fn init_rand(&mut self, edge: bool) {
        let (i_min, i_max) = if edge { (0, self.m )} else { (1, self.m - 1) };
        let (j_min, j_max) = if edge { (0, self.n) } else { (1, self.n - 1) };
        for i in i_min..i_max {
            for j in j_min..j_max {
                self.cells[i][j] = rand::thread_rng().gen_range(0..self.q);
            }
        }  
    }

    pub fn next(&mut self) {
        let (i_min, i_max) = if self.torus {(0, self.m)} else {(1, self.m - 1)};
        let (j_min, j_max) = if self.torus {(0, self.n)} else {(1, self.n - 1)};
        for i in i_min..i_max {
            for j in j_min..j_max {
                self.temp[i][j] = (self.next_state)(self, i, j);
            }
        }
        self.swap_buffer();
    }

    fn swap_buffer(&mut self) {
        swap(&mut self.cells, &mut self.temp);
    }
}
