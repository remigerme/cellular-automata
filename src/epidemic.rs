use rand::Rng;
use probability::{distribution::Gaussian, prelude::Continuous};
use crate::automaton::Automaton;


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum State {
    HEALTHY,
    INFECTED(u32),
    RECOVERED,
    DEAD    
}

use self::State::*;

fn next_state(a: &Automaton<State>, i: usize, j: usize) -> State {
    let neighbours = a.get_von_neumann_neighbours(i, j);
    let nb_infected = neighbours
        .iter()
        .filter(|&x| match *x {
            INFECTED(_) => true,
            _ => false
        })
        .count();
    let mut rng = rand::thread_rng();
    // This doesn't follow any real model
    // Just wanted to try it out
    let p_natural_infection = 0.;
    let p_infection_per_neighbour = 0.09;
    let day_recover_spike_1 = 8;
    let day_death_spike = 10;
    // symmetry
    let day_recover_spike_2 = day_death_spike + (day_death_spike - day_recover_spike_1);
    let sigma = 2.;

    match a.get_cell(i, j) {
        HEALTHY => if nb_infected == 0 && rng.gen::<f64>() < p_natural_infection
                    || rng.gen::<f64>() < nb_infected as f64 * p_infection_per_neighbour {
            INFECTED(0)
        } else {
            HEALTHY
        },
        INFECTED(x) => {
            // Gaussian recover distribution 1 and 2
            let gr1 = Gaussian::new(day_recover_spike_1 as f64, sigma);
            let gr2 = Gaussian::new(day_recover_spike_2 as f64, sigma);
            // Gaussian death distribution
            let gd = Gaussian::new(day_death_spike as f64, sigma);

            // We need to find the max value of the sum of the previous gaussians
            // Bc we need to normalize for this sum don't exceed 1
            let sum_of_gaussian = |x| {
                gr1.density(x) + gr2.density(x) + gd.density(x)
            };
            let normalization_constant = max_value_fn(
                Box::new(sum_of_gaussian), 
                0.,
                day_recover_spike_2 as f64 * 2.
            );

            let bound_recover = (gr1.density(x as f64) + gr2.density(x as f64)) / normalization_constant;
            let bound_death = bound_recover + gd.density(x as f64) / normalization_constant;
            let e = rng.gen::<f64>();

            if e < bound_recover {
                RECOVERED
            } else if e < bound_death {
                DEAD
            } else {
                INFECTED(x + 1)
            }
        },
        RECOVERED => RECOVERED,
        DEAD => DEAD
    }
}


pub fn new(m: usize, n: usize, torus: bool) -> Automaton<State> {
    let states = vec![HEALTHY, INFECTED(0), RECOVERED, DEAD];
    let get_color = |s| match s {
        HEALTHY => 0xFFFFFF,
        INFECTED(_) => 0xFF0000,
        RECOVERED => 0x00FF00,
        DEAD => 0
    };
    let mut cells = vec![vec![HEALTHY; n]; m];
    cells[m / 2][n / 2] = INFECTED(0); // One infected
    Automaton::new(
        m,
        n,
        4,
        states,
        torus,
        Box::new(next_state),
        Box::new(get_color),
        cells
    )
}


fn max_value_fn<T>(f: T, a: f64, b: f64) -> f64
where
    T: Fn(f64) -> f64
{
    let n = 1000;
    let h = (b - a) / (n as f64 - 1.);
    let absc = (0..n)
        .map(|i| a + i as f64 * h);
    absc
        .map(|x| f(x))
        .reduce(f64::max)
        .unwrap()
}
