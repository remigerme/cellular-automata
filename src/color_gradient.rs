use std::collections::HashMap;
use crate::automaton::Automaton;


fn next_state(a: &Automaton<u32>, i: usize, j: usize) -> u32 {
    let mut neighbours = a.get_moore_neighbours(i, j);
    neighbours.push(a.get_cell(i, j));
    let sum = neighbours.iter().fold(0., |s, x| s + *x as f32);
    let x = (sum / 9.) as u32;
    (x + 1) % a.get_q()
}


pub fn new(m: usize, n: usize, torus: bool) -> Automaton<u32> {
    let colors: HashMap<u32, u32> = 
        (0..360)
            .map(|x| (x, x))
            .map(|(x, y)| (x, hsv_to_hex((y as f32, 1.0, 1.0))))
            .collect();
    let states = colors
            .keys()
            .map(|x| *x)
            .collect();
    Automaton::<u32>::new(
        m,
        n,
        360,
        states,
        torus,
        Box::new(next_state),
        colors,
        vec![vec![0; n]; m]
    )
}


// utils to move in another file later ?
fn float_modulo(x: f32, y: f32) -> f32 {
    // we must have y > 0
    let mut x_ = x;
    let eps = if x_ >= 0. {-1.} else {1.};
    while x_ < 0. || x_ > y as f32 {
        x_ += eps * y;
    }
    x_
}


fn hsv_to_rgb(hsv: (f32, f32, f32)) -> (u8, u8, u8) {
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


fn rgb_to_hex(rgb: (u8, u8, u8)) -> u32 {
    rgb.0 as u32 * 65536 + rgb.1 as u32 * 256 + rgb.2 as u32
}


fn hsv_to_hex(hsv: (f32, f32, f32)) -> u32 {
    rgb_to_hex(hsv_to_rgb(hsv))
}