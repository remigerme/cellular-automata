use std::vec::Vec;
use std::hash::Hash;
use std::mem::swap;
use image::{ImageBuffer, RgbImage, Rgb};
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::io::{stdout, Write};
use crate::color::hex_to_rgb;


pub struct Automaton1D<T>
where
    T: Copy + Eq + Hash
{
    n: usize,
    q: u32,
    states: Vec<T>,
    torus: bool,
    next_state: Box<dyn Fn(&Self, usize) -> T>,
    get_color: Box<dyn Fn(T) -> u32>,
    cells: Vec<T>,
    temp: Vec<T>
}


impl<T> Automaton1D<T>
where
    T: Copy + Eq + Hash
{
    pub fn new(
        n: usize,
        q: u32,
        states: Vec<T>,
        torus: bool,
        next_state: Box<dyn Fn(&Automaton1D<T>, usize) -> T>,
        get_color: Box<dyn Fn(T) -> u32>,
        cells: Vec<T>
    ) -> Self {
        Automaton1D {
            n, q, states, torus, next_state, get_color, cells: cells.clone(), temp: cells
        }
    }

    // Getters
    pub fn get_size(&self) -> usize { self.n }
    pub fn get_q(&self) -> u32 { self.q }
    pub fn get_cells(&self) -> Vec<T> { self.cells.clone() }
    pub fn get_cell(&self, i: usize) -> T {self.cells[i] }
    pub fn get_cell_color(&self, i: usize) -> u32 { (self.get_color)(self.cells[i]) }

    pub fn get_neighbours(&self, i: usize) -> Vec<T> {
        // Including the cell itself
        vec![
            self.cells[(i - 1) % self.n],
            self.cells[i],
            self.cells[(i + 1) % self.n]
        ]
    }

    // Init
    pub fn init_state(&mut self, s: T, edge: bool) {
        let (i_min, i_max) = if edge { (0, self.n )} else { (1, self.n - 1) };
        for i in i_min..i_max {
            self.cells[i] = s;
        }
    }

    pub fn init_rand(&mut self, edge: bool) {
        let (i_min, i_max) = if edge { (0, self.n )} else { (1, self.n - 1) };
        let mut rng = thread_rng();
        for i in i_min..i_max {
            self.cells[i] = match self.states.choose(&mut rng) {
                Some(s) => *s,
                _ => panic!("Automaton has no possible state")
           };
       }
    }

    pub fn next(&mut self) {
        let (i_min, i_max) = if self.torus {(0, self.n)} else {(1, self.n - 1)};
        for i in i_min..i_max {
            self.temp[i] = (self.next_state)(self, i);
        }
        self.swap_buffer();
    }

    fn swap_buffer(&mut self) {
        swap(&mut self.cells, &mut self.temp);
    }
}


pub fn generate_image<T>(width: usize, height: usize, cell_size: usize, a: &mut Automaton1D<T>) -> RgbImage
where
    T: Copy + Eq + Hash
{
    let mut img = ImageBuffer::new(width as u32, height as u32);
    let m = height / cell_size;
    let mut per_one = 0.0;
    print!("Progression (# = 10%) : ");
    for k in 0..m {
        if (k as f64) / (m as f64) > per_one {
            per_one += 0.1;
            print!("#");
            stdout().flush().unwrap();
        }
        for y in (k * cell_size)..((k + 1) * cell_size) {
            for x in 0..width {
                let hex_color = a.get_cell_color(x);
                let pixel = Rgb(hex_to_rgb(hex_color));
                img.put_pixel(x as u32, y as u32, pixel)
            }
        }
        a.next();
    }
    println!("");
    img 
} 


pub fn update_buffer<T>(buffer: &mut Vec<u32>, a: &Automaton1D<T>, k: usize, width: usize, cell_size: usize)
where
    T: Copy + Eq + Hash
{
    let i_min = k * width * cell_size;
    let i_max = (k + 1) * width * cell_size;
    // looks slower than to iter over the whole mut buffer
    // as done for automaton 2D
    // but don't know why
    for i in i_min..i_max {
        buffer[i] = a.get_cell_color((i % width) / cell_size);
    }

}
