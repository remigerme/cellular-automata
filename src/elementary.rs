use crate::automaton_1d::Automaton1D;


pub fn new(n: usize, rule: &'static u8, torus: bool) -> Automaton1D<bool> {
    let next_state = |a: &Automaton1D<bool>, i: usize| -> bool {
        let neighbours = a.get_neighbours(i);
        let config = 4 * neighbours[0] as u32 + 2 * neighbours[1] as u32 + neighbours[2] as u32;
        ((*rule >> config) & 0b1) != 0
    };
    let mut cells = vec![false; n];
    cells[n / 2] = true;
    Automaton1D::new(
        n,
        2,
        vec![false, true],
        torus,
        Box::new(next_state),
        Box::new(|x| match x {
            false => 0xFFFFFF,
            true => 0,
        }),
        cells
    )
}
