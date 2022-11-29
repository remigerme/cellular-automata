use rand::Rng;
use crate::automaton::Automaton;


const ROCK: u32 = 0;
const GREY: u32 = 0xB0B0B0;
const ASHES: u32 = 1;
const BLACK: u32 = 0x3B3737;
const YOUNG: u32 = 2;
const GREEN: u32 = 0x68E180;
const OLD: u32 = 3;
const DARK_GREEN: u32 = 0x24A609;
const NEW_FIRE: u32 = 4;
const YELLOW: u32 = 0xF2F26E;
const FIRE: u32 = 5;
const ORANGE: u32 = 0xE48F01;
const FADING_FIRE: u32 = 6;
const RED: u32 = 0xBA2E0B;


fn next_state(a: &Automaton, i: usize, j: usize) -> u32 {
    let mut rng = rand::thread_rng();
    match a.get_cell(i, j) {
        ROCK => ROCK,
        ASHES => if rng.gen::<f64>() < 0.001 {YOUNG} else {ASHES},
        YOUNG => {
            let neighbours = a.get_neighbours(i, j);
            if rng.gen::<f64>() < 0.01 && neighbours.contains(&NEW_FIRE) {
                NEW_FIRE
            } else if rng.gen::<f64>() < 0.02 && neighbours.contains(&FIRE) {
                NEW_FIRE
            } else if rng.gen::<f64>() < 0.01 && neighbours.contains(&FADING_FIRE) {
                NEW_FIRE
            } else if rng.gen::<f64>() < 0.005 {
                OLD
            } else {
                YOUNG
            }
        },
        OLD => {
            let neighbours = a.get_neighbours(i, j);
            if rng.gen::<f64>() < 0.1 && neighbours.contains(&NEW_FIRE) {
                NEW_FIRE
            } else if rng.gen::<f64>() < 0.2 && neighbours.contains(&FIRE) {
                NEW_FIRE
            } else if rng.gen::<f64>() < 0.1 && neighbours.contains(&FADING_FIRE) {
                NEW_FIRE
            } else if rng.gen::<f64>() < 0.00005 && neighbours.iter().filter(|&n| *n == OLD).count() >= 5 {
                NEW_FIRE
            } else {
                OLD
            }
        },
        NEW_FIRE => if rng.gen::<f64>() < 0.1 {FIRE} else {NEW_FIRE},
        FIRE => if rng.gen::<f64>() < 0.1 {FADING_FIRE} else {FIRE},
        FADING_FIRE => if rng.gen::<f64>() < 0.1 {ASHES} else {FADING_FIRE},
        _ => ROCK,
    }
}


pub fn new(m: usize, n: usize, torus: bool) -> Automaton {
    Automaton::new(
        m,
        n,
        7,
        torus,
        Box::new(next_state),
        vec![GREY, BLACK, GREEN, DARK_GREEN, YELLOW, ORANGE, RED],
        vec![vec![ROCK; n]; m] 
    )
}
