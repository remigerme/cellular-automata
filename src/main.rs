use std::vec::Vec;
use std::mem::swap;

use rand::Rng;

use minifb::{Key, Window, WindowOptions};

const M: usize = 800;
const N: usize = 1200;

const CELL_SIZE: usize = 1;

const WIDTH: usize = N * CELL_SIZE;
const HEIGHT: usize = M * CELL_SIZE;

struct Automaton {
    m: usize,
    n: usize,
    cells: Vec<Vec<u32>>,
    temp: Vec<Vec<u32>>,
}

impl Automaton {
    fn init_rand(&mut self) {
        for i in 1..self.m - 1 {
            for j in 1..self.n - 1 {
                let e = rand::thread_rng().gen_range(0..=1);
                self.cells[i][j] = e;
                self.temp[i][j] = e;
            }
        }
    }

    fn init_state(&mut self, state: u32) {
        for i in 1..self.m - 1 {
            for j in 1..self.n - 1 {
                self.cells[i][j] = state;
                self.temp[i][j] = state;
            }
        }
    }

    fn swap_buffer(&mut self) {
        swap(&mut self.cells, &mut self.temp);
    }

    fn get_neighbours(&self, i: usize, j: usize) -> Vec<u32> {
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

    fn new_state(&self, i: usize, j: usize) -> u32 {
        let neighbours = self.get_neighbours(i, j);
        let nb_neighbours_alive = neighbours.iter().fold(0, |a, b| a + b);
    
        match self.cells[i][j] {
            0 => if nb_neighbours_alive == 3 {1} else {0},
            1 => if nb_neighbours_alive == 2 || nb_neighbours_alive == 3 {1} else {0},
            _ => 0
        }
    }

    fn next(&mut self) {
        for i in 1..self.m - 1 {
            for j in 1..self.n - 1 {
                self.temp[i][j] = self.new_state(i, j);
            }
        }
        self.swap_buffer();
    }
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    
    let mut window = Window::new(
        "Cellular automaton - ESC to quit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));


    let mut automaton = Automaton {
        m: M,
        n: N,
        cells: vec![vec![0u32; WIDTH]; HEIGHT],
        temp: vec![vec![0u32; WIDTH]; HEIGHT],
    };
    automaton.init_rand();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        automaton.next();

        for (index, cell) in buffer.iter_mut().enumerate() {
            let x = index / WIDTH;
            let y = index % WIDTH;

            let cell_x = x / CELL_SIZE;
            let cell_y = y / CELL_SIZE;

            *cell = match automaton.cells[cell_x][cell_y] {
                0 => 0xFFFFFF,
                1 => 0,
                _ => 0
            };
        }
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
