use crate::automaton_2d::Automaton2D;
use crate::color::hsv_to_hex;


fn next_state(a: &Automaton2D<u32>, i: usize, j: usize) -> u32 {
    let mut neighbours = a.get_moore_neighbours(i, j);
    neighbours.push(a.get_cell(i, j));
    let sum = neighbours.iter().fold(0., |s, x| s + *x as f32);
    let x = (sum / 9.) as u32;
    (x + 1) % a.get_q()
}


pub fn new(m: usize, n: usize, torus: bool) -> Automaton2D<u32> {
    let get_color = |s| hsv_to_hex([s as f64, 1.0, 1.0]);
    let states = (0..360).collect();
    Automaton2D::<u32>::new(
        m,
        n,
        360,
        states,
        torus,
        Box::new(next_state),
        Box::new(get_color),
        vec![vec![0; n]; m]
    )
}
