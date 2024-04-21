use ndarray::Array;

const C_M: f64 = 1.0;
const G_NA_MAX: f64 = 120.0;
const G_K_MAX: f64 = 36.0;
const G_L: f64 = 0.3;
const V_NA: f64 = 115.0;
const V_K: f64 = -12.0;
const V_L: f64 = 10.613;
const V_REST: f64 = -65.0;

pub struct HHOutput {
    pub t: f64,
    pub i_s: f64,
    pub vm: f64,
    pub g_na: f64,
    pub i_na: f64,
    pub g_k: f64,
    pub i_k: f64,
    pub g_l: f64,
    pub i_l: f64,
    pub i_total: f64,
}

pub fn hh_initial_state() -> Array<f64, ndarray::Ix1> {
    Array::from(vec![
        -65.0,
        5.293230752317206e-02,
        5.961139478306510e-01,
        3.177526546989322e-01,
    ])
} 

pub fn hh_derivatives<F>(
    state: &Array<f64, ndarray::Ix1>,
    t: f64,
    current: F,
) -> Array<f64, ndarray::Ix1>
where
    F: Fn(f64) -> f64,
{
    let v = state[0] - V_REST;
    let m = state[1];
    let h = state[2];
    let n = state[3];
    let i = current(t);

    let alpha_m = 0.1 * (25.0 - v) / (((25.0 - v) / 10.0).exp() - 1.0);
    let alpha_h = 0.07 / (v / 20.0).exp();
    let alpha_n = 0.01 * (10.0 - v) / (((10.0 - v) / 10.0).exp() - 1.0);
    let beta_m = 4.0 / (v / 18.0).exp();
    let beta_h = 1.0 / (((30.0 - v) / 10.0).exp() + 1.0);
    let beta_n = 0.125 / (v / 80.0).exp();

    let dv = (i
        - (v - V_NA) * G_NA_MAX * m.powi(3) * h
        - (v - V_K) * G_K_MAX * n.powi(4)
        - (v - V_L) * G_L)
        / C_M;
    let dm = alpha_m * (1.0 - m) - beta_m * m;
    let dh = alpha_h * (1.0 - h) - beta_h * h;
    let dn = alpha_n * (1.0 - n) - beta_n * n;

    Array::from(vec![dv, dm, dh, dn])
}

pub fn hh_output(state: &Array<f64, ndarray::Ix1>, t: f64, i: f64) -> HHOutput {
    let v = state[0] - V_REST;
    let m = state[1];
    let h = state[2];
    let n = state[3];

    HHOutput {
        t,
        i_s: i,
        vm: state[0],
        g_na: G_NA_MAX * m.powi(3) * h,
        i_na: G_NA_MAX * m.powi(3) * h * (v - V_NA),
        g_k: G_K_MAX * n.powi(4),
        i_k: G_K_MAX * n.powi(4) * (v - V_K),
        g_l: G_L,
        i_l: G_L * (v - V_L),
        i_total: G_NA_MAX * m.powi(3) * h * (v - V_NA)
            + G_K_MAX * n.powi(4) * (v - V_K)
            + G_L * (v - V_L),
    }
}
