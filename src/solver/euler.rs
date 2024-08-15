use super::hh::*;
use ndarray::Array;

fn euler<F>(
    state: &Array<f64, ndarray::Ix1>,
    t: f64,
    dt: f64,
    current: F,
) -> Array<f64, ndarray::Ix1>
where
    F: Fn(f64) -> f64,
{
    let k = hh_derivatives(state, t, &current);
    state + &k * dt
}

pub fn hh_simulation_euler(points: Vec<(f64, f64)>, dt: f64, t_max: f64) -> Vec<HHOutput> {
    let mut state = hh_initial_state();
    let mut t = 0.0;
    let mut data = Vec::<HHOutput>::new();

    let hh_current = |t: f64| -> f64 {
        let mut i = 0.0;
        for (t_i, i_i) in &points {
            if t >= *t_i {
                i = *i_i;
            }
        }
        i
    };
    while t < t_max {
        state = euler(&state, t, dt, hh_current);
        let output = hh_output(&state, t, hh_current(t));
        t += dt;
        data.push(output);
    }

    data
}
