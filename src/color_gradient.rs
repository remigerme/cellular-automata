use crate::automaton::{Automaton, Access, Init, Rules};


pub struct ColorGradientAutomaton(Automaton);


impl ColorGradientAutomaton {
    pub fn new(m: usize, n: usize, torus: bool, cells: Vec<Vec<u32>>) -> Self {
        let q = 360;
        let colors = (0..q).map(|x| rgb_to_hex(hsv_to_rgb((x as f32, 1.0, 1.0)))).collect::<Vec<u32>>();
        ColorGradientAutomaton(Automaton::new(m, n, q, torus, colors, cells))
    }
}


impl Rules for ColorGradientAutomaton {
    fn next_state(&self, i: usize, j: usize) -> u32 {
        let mut neighbours = self.0.get_neighbours(i, j);
        neighbours.push(self.0.cells[i][j]);
        let sum = neighbours.iter().fold(0., |s, x| s + *x as f32);
        let x = (sum / 9.) as u32;
        (x + 1) % self.0.q
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
impl Access for ColorGradientAutomaton {
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


impl Init for ColorGradientAutomaton {
    fn init_rand(&mut self) {
        self.0.init_rand()
    }

    fn init_state(&mut self, s: u32) {
        self.0.init_state(s)
    }
}


// Not delegation : utils to move in another file later
fn float_modulo(x: f32, y: f32) -> f32 {
    // we must have y > 0
    let mut x_ = x;
    let eps = if x_ >= 0. {-1.} else {1.};
    while x_ < 0. || x_ > y as f32 {
        x_ += eps * y;
    }
    x_
}


pub fn hsv_to_rgb(hsv: (f32, f32, f32)) -> (u8, u8, u8) {
    let (h, s, v) = hsv;
    
    let c = v * s;
    let h_ = h / 60.;
    let x = c * (1. - (float_modulo(h_, 2.) - 1.).abs());

    let (r, g, b) = {
        let (r1, g1, b1) = {
            if h_ < 1. {
                (c, x, 0.)
            } else if h_ < 2. {
                (x, c, 0.)
            } else if h_ < 3. {
                (0., c, x)
            } else if h_ < 4. {
                (0., x, c)
            } else if h_ < 5. {
                (x, 0., c)
            } else {
                (c, 0., x)
            }
        };
        let m = v - c;
        let (r, g, b) = (r1 + m, g1 + m, b1 + m);
        ((r * 255.) as u8, (g * 255.) as u8, (b * 255.) as u8)
    };
    (r, g, b)
}


pub fn rgb_to_hex(rgb: (u8, u8, u8)) -> u32 {
    rgb.0 as u32 * 65536 + rgb.1 as u32 * 256 + rgb.2 as u32
}
