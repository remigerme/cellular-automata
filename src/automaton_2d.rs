use std::vec::Vec;
use std::hash::Hash;
use std::mem::swap;
use rand::thread_rng;
use rand::seq::SliceRandom;


pub struct Automaton2D<T>
where
    T: Copy + Eq + Hash
{
    m: usize,
    n: usize,
    q: u32,
    states: Vec<T>,
    torus: bool,
    next_state: Box<dyn Fn(&Self, usize, usize) -> T>,
    get_color: Box<dyn Fn(T) -> u32>,
    cells: Vec<Vec<T>>,
    temp: Vec<Vec<T>>
}


impl<T> Automaton2D<T>
where
    T: Copy + Eq + Hash
{
    pub fn new(
        m: usize,
        n: usize,
        q: u32,
        states: Vec<T>,
        torus: bool,
        next_state: Box<dyn Fn(&Automaton2D<T>, usize, usize) -> T>,
        get_color: Box<dyn Fn(T) -> u32>,
        cells: Vec<Vec<T>>
    ) -> Self {
        Automaton2D {
            m, n, q, states, torus, next_state, get_color, cells: cells.clone(), temp: cells
        }
    }

    // Getters
    pub fn get_size(&self) -> (usize, usize) { (self.m, self.n) }
    pub fn get_q(&self) -> u32 { self.q }
    pub fn get_cells(&self) -> Vec<Vec<T>> { self.cells.clone() }
    pub fn get_cell(&self, i: usize, j: usize) -> T {self.cells[i][j] }
    pub fn get_cell_color(&self, i: usize, j: usize) -> u32 { (self.get_color)(self.cells[i][j]) }

    pub fn get_von_neumann_neighbours(&self, i: usize, j: usize) -> Vec<T> {
        let m = self.m;
        let n = self.n;
        vec![
            self.cells[(i - 1) % m][j],
            self.cells[i][(j + 1) % n],
            self.cells[(i + 1) % m][j],
            self.cells[i][(j - 1) % n]
        ]
    }

    pub fn get_moore_neighbours(&self, i: usize, j: usize) -> Vec<T> {
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
    pub fn init_state(&mut self, s: T, edge: bool) {
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
       let mut rng = thread_rng();
       for i in i_min..i_max {
           for j in j_min..j_max {
               self.cells[i][j] = match self.states.choose(&mut rng) {
                    Some(s) => *s,
                    _ => panic!("Automaton2D has no possible state")
               };
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
