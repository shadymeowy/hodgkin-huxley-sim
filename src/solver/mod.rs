pub mod dopri;
pub mod euler;
pub mod hh;
pub mod rk4;

pub use dopri::*;
pub use euler::*;
pub use hh::*;
pub use rk4::*;

#[derive(Debug, Clone, Copy)]
pub enum SolverType {
    RK4,
    Dopri,
    Euler,
}

pub fn hh_simulation(
    points: Vec<(f64, f64)>,
    dt: f64,
    duration: f64,
    solver: SolverType,
) -> Vec<HHOutput> {
    match solver {
        SolverType::RK4 => hh_simulation_rk4(points, dt, duration),
        SolverType::Dopri => hh_simulation_dopri(points, dt, duration),
        SolverType::Euler => hh_simulation_euler(points, dt, duration),
    }
}
