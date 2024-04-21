use super::hh::*;
use ndarray::Array;

fn dopri_step<F>(
    state: &Array<f64, ndarray::Ix1>,
    t: f64,
    dt: f64,
    current: F,
) -> (Array<f64, ndarray::Ix1>, f64)
where
    F: Fn(f64) -> f64,
{
    // Coefficients for the Dormand-Prince method
    let k1 = hh_derivatives(state, t, &current);
    let k2 = hh_derivatives(
        &(state + &(&k1 * (dt * 1.0 / 5.0))),
        t + dt * 1.0 / 5.0,
        &current,
    );
    let k3 = hh_derivatives(
        &(state + &(&k1 * (dt * 3.0 / 40.0)) + &(&k2 * (dt * 9.0 / 40.0))),
        t + dt * 3.0 / 10.0,
        &current,
    );
    let k4 = hh_derivatives(
        &(state + &(&k1 * (dt * 44.0 / 45.0)) - &(&k2 * (dt * 56.0 / 15.0))
            + &(&k3 * (dt * 32.0 / 9.0))),
        t + dt * 4.0 / 5.0,
        &current,
    );
    let k5 = hh_derivatives(
        &(state + &(&k1 * (dt * 19372.0 / 6561.0)) - &(&k2 * (dt * 25360.0 / 2187.0))
            + &(&k3 * (dt * 64448.0 / 6561.0))
            - &(&k4 * (dt * 212.0 / 729.0))),
        t + dt * 8.0 / 9.0,
        &current,
    );
    let k6 = hh_derivatives(
        &(state + &(&k1 * (dt * 9017.0 / 3168.0)) - &(&k2 * (dt * 355.0 / 33.0))
            + &(&k3 * (dt * 46732.0 / 5247.0))
            + &(&k4 * (dt * 49.0 / 176.0))
            - &(&k5 * (dt * 5103.0 / 18656.0))),
        t + dt,
        &current,
    );

    let k7 = hh_derivatives(
        &(state
            + &(&k1 * (dt * 35.0 / 384.0))
            + &(&k3 * (dt * 500.0 / 1113.0))
            + &(&k4 * (dt * 125.0 / 192.0))
            - &(&k5 * (dt * 2187.0 / 6784.0))
            + &(&k6 * (dt * 11.0 / 84.0))),
        t + dt,
        &current,
    );

    let y_fifth = state
        + &(&k1 * (dt * 35.0 / 384.0))
        + &(&k3 * (dt * 500.0 / 1113.0))
        + &(&k4 * (dt * 125.0 / 192.0))
        - &(&k5 * (dt * 2187.0 / 6784.0))
        + &(&k6 * (dt * 11.0 / 84.0));
    let y_fourth = state
        + &(&k1 * (dt * 5179.0 / 57600.0))
        + &(&k3 * (dt * 7571.0 / 16695.0))
        + &(&k4 * (dt * 393.0 / 640.0))
        - &(&k5 * (dt * 92097.0 / 339200.0))
        + &(&k6 * (dt * 187.0 / 2100.0))
        + &(&k7 * (dt * 1.0 / 40.0));

    let error = &y_fifth - y_fourth;
    let max_error = error.iter().map(|x| x.abs()).fold(0.0, f64::max);

    (y_fifth, max_error)
}

pub fn hh_simulation_dopri(points: Vec<(f64, f64)>, dt: f64, t_max: f64) -> Vec<HHOutput> {
    let mut state = hh_initial_state();
    let mut t = 0.0;
    let dt_min = dt * 1e-6;
    let dt_max = dt;
    let mut dt = dt;
    let mut data = Vec::<HHOutput>::new();
    let tolerance = 0.1;

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
        if t + dt > t_max {
            dt = t_max - t;
        }
        let (state_next, error) = dopri_step(&state, t, dt, hh_current);
        if error <= tolerance {
            t += dt;
            state = state_next;
            let output = hh_output(&state, t, hh_current(t));
            data.push(output);
            dt = dt_max.min(dt * (tolerance / error).powf(1.0 / 5.0));
        } else {
            dt = dt_min.max(dt * (tolerance / error).powf(1.0 / 5.0));
        }
    }
    data
}
