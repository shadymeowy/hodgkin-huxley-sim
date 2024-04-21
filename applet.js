function cb_example() {
    let select_example = document.getElementById("select-example");
    let txt_points = document.getElementById("txt-points");
    let duration = document.getElementById("input-duration");
    let timesteps = document.getElementById("input-timestep");
    timesteps.value = 0.1;
    select_solver.value = "Dormand-Prince";
    scenarios = {
        "Single action potential": [30, "10, 1, 20"],
        "Multiple action potentials": [70, "10, 2, 15\n20, 2, 15\n20, 2, 15"],
        "Refactory period": [40, "10, 5, 5\n5, 5, 10"],
        "Accommodation": [40, "10, 25, 5"],
        "Pulse-width modulation": [200, "20, 80, 10\n0, 80, 40"],
        "Temporal summation": [40, "5, 0.3, 20\n10, 0.3, 20\n2, 0.3, 20\n14, 0.3, 20\n5, 0.3, 20"]
    };
    duration.value = scenarios[select_example.value][0];
    txt_points.value = scenarios[select_example.value][1];
    applet.set_solver("Dormand-Prince");
    applet.set_duration(duration.value);
    applet.set_timestep(timesteps.value);
    applet.plot();
}

function cb_solver() {
    let select_solver = document.getElementById("select-solver");
    applet.set_solver(select_solver.value);
    if (select_solver.value == "Dormand-Prince") {
        timesteps.value = 0.1; // Lower timestep for Dormand-Prince
    }
    else {
        timesteps.value = 0.01; // Higher timestep for Euler and Runge-Kutta
    }
    applet.set_timestep(timesteps.value);
    applet.plot();
}

let plot_div = document.getElementById("div-plot");
let txt_points = document.getElementById("txt-points");
let duration = document.getElementById("input-duration");
let timesteps = document.getElementById("input-timestep");
let select_solver = document.getElementById("select-solver");
let btn_update = document.getElementById("btn-update");
let select_example = document.getElementById("select-example");
duration.addEventListener("input", () => applet.set_duration(duration.value));
duration.addEventListener("change", () => applet.set_duration(duration.value));
timesteps.addEventListener("input", () => applet.set_timestep(timesteps.value));
timesteps.addEventListener("change", () => applet.set_timestep(timesteps.value));
btn_update.addEventListener("click", () => applet.plot());
select_example.addEventListener("change", cb_example);
select_solver.addEventListener("change", cb_solver);
txt_points.value = "30, 10, 5";

async function run() {
    await wasm_bindgen();
    let applet = wasm_bindgen.APApplet.new(plot_div, txt_points, 100);
    window.applet = applet;
    window.SolverType = wasm_bindgen.SolverType;
    cb_example();
    cb_solver();
}
run(); 
