use crate::solver::*;
use plotly::common::Title;
use plotly::layout::GridPattern;
use plotly::layout::LayoutGrid;
use plotly::Layout;
use plotly::{Plot, Scatter};
use std::f64;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[allow(unused_macros)]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format!($($t)*)))
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct APApplet {
    div_plot: web_sys::HtmlElement,
    text_area: web_sys::HtmlTextAreaElement,
    duration: f64,
    solver: SolverType,
    dt: f64,
}

#[wasm_bindgen]
impl APApplet {
    pub fn new(
        div_plot: web_sys::HtmlElement,
        text_area: web_sys::HtmlTextAreaElement,
        duration: f64,
    ) -> Self {
        Self {
            div_plot,
            text_area,
            duration,
            solver: SolverType::Dopri,
            dt: 0.1,
        }
    }

    pub fn plot(&self) -> Result<(), JsValue> {
        let points = self.read_data_points().expect("Invalid data points");

        let mut t = Vec::<f64>::new();
        let mut i_s = Vec::<f64>::new();
        let mut vm = Vec::<f64>::new();
        let mut g_na = Vec::<f64>::new();
        let mut i_na = Vec::<f64>::new();
        let mut g_k = Vec::<f64>::new();
        let mut i_k = Vec::<f64>::new();
        let mut g_l = Vec::<f64>::new();
        let mut i_l = Vec::<f64>::new();
        let mut i_total = Vec::<f64>::new();

        let data = hh_simulation(points, self.dt, self.duration, self.solver);

        for d in data {
            t.push(d.t);
            vm.push(d.vm);
            i_s.push(d.i_s);
            g_na.push(d.g_na);
            i_na.push(d.i_na);
            g_k.push(d.g_k);
            i_k.push(d.i_k);
            g_l.push(d.g_l);
            i_l.push(d.i_l);
            i_total.push(d.i_total);
        }

        console_log!("data length: {} dt: {}", t.len(), self.dt);

        let mut plot = Plot::new();
        let conf = plot
            .configuration()
            .clone()
            .typeset_math(true)
            .responsive(true);
        plot.set_configuration(conf);
        let scatter_v = Scatter::new(t.clone(), vm)
            .name("$V_m (mV)$")
            .x_axis("x")
            .y_axis("y");
        plot.add_trace(scatter_v);
        let scatter_i = Scatter::new(t.clone(), i_s)
            .name("$I_s (\\mu A)$")
            .x_axis("x")
            .y_axis("y5");
        plot.add_trace(scatter_i);
        let scatter_g_na = Scatter::new(t.clone(), g_na)
            .name("$g_{Na} (mS/cm^2)$")
            .x_axis("x2")
            .y_axis("y2");
        plot.add_trace(scatter_g_na);
        let scatter_g_k = Scatter::new(t.clone(), g_k)
            .name("$g_K (mS/cm^2)$")
            .x_axis("x2")
            .y_axis("y2");
        plot.add_trace(scatter_g_k);
        let scatter_g_l = Scatter::new(t.clone(), g_l)
            .name("$g_L (mS/cm^2)$")
            .x_axis("x2")
            .y_axis("y2");
        plot.add_trace(scatter_g_l);
        let scatter_i_na = Scatter::new(t.clone(), i_na)
            .name("$I_{Na} (\\mu A)$")
            .x_axis("x3")
            .y_axis("y3");
        plot.add_trace(scatter_i_na);
        let scatter_i_k = Scatter::new(t.clone(), i_k)
            .name("$I_K (\\mu A)$")
            .x_axis("x3")
            .y_axis("y3");
        plot.add_trace(scatter_i_k);
        let scatter_i_l = Scatter::new(t.clone(), i_l)
            .name("$I_L (\\mu A)$")
            .x_axis("x3")
            .y_axis("y3");
        plot.add_trace(scatter_i_l);
        let scatter_i_total = Scatter::new(t.clone(), i_total)
            .name("$I_{total} (\\mu A)$")
            .x_axis("x3")
            .y_axis("y3");
        plot.add_trace(scatter_i_total);

        let layout = Layout::new()
            .grid(
                LayoutGrid::new()
                    .rows(3)
                    .columns(1)
                    .pattern(GridPattern::Independent),
            )
            .title(Title::with_text("Hodgkin-Huxley Model Simulation"))
            .x_axis(plotly::layout::Axis::new())
            .x_axis3(plotly::layout::Axis::new().title(Title::with_text("$t$ ($ms$)")))
            .y_axis(
                plotly::layout::Axis::new()
                    .title(Title::with_text("Voltage ($mV$)"))
                    .side(plotly::common::AxisSide::Left),
            )
            .y_axis2(
                plotly::layout::Axis::new()
                    .title(Title::with_text("Conductance ($mS/cm^2$)"))
                    .anchor("x2"),
            )
            .y_axis3(
                plotly::layout::Axis::new()
                    .title(Title::with_text("Current ($\\mu A$)"))
                    .anchor("x3"),
            )
            .y_axis5(
                plotly::layout::Axis::new()
                    .title(Title::with_text("Current ($\\mu A$)"))
                    .overlaying("y")
                    .side(plotly::common::AxisSide::Right)
                    .range(vec![-10.0, 40.0]),
            )
            .legend(plotly::layout::Legend::new().x(0.9).y(0.1));
        plot.set_layout(layout);

        self.add_plot(plot)?;
        Ok(())
    }

    pub fn set_duration(&mut self, duration: f64) {
        self.duration = duration;
    }

    pub fn set_timestep(&mut self, dt: f64) {
        self.dt = dt;
    }

    pub fn set_solver(&mut self, solver: &str) {
        match solver {
            "Runge-Kutta" => self.solver = SolverType::RK4,
            "Dormand-Prince" => self.solver = SolverType::Dopri,
            "Euler" => self.solver = SolverType::Euler,
            _ => self.solver = SolverType::Dopri,
        }
    }

    fn run_script(&self, script: &str) -> Result<(), JsValue> {
        web_sys::js_sys::eval(script)?;
        Ok(())
    }

    fn add_plot(&self, plot: Plot) -> Result<(), JsValue> {
        let html = plot.to_inline_html(Some("div-plot-inside"));
        self.div_plot.set_inner_html(&html);
        let scripts = self.div_plot.query_selector_all("script")?;
        for i in 0..scripts.length() {
            let script = scripts.get(i).ok_or("No script found")?;
            let script = script.dyn_into::<web_sys::HtmlScriptElement>()?;
            let content = script.text()?;
            self.run_script(&content.to_string())?;
        }
        Ok(())
    }

    fn parse_data_points(&self, s: &str) -> Result<Vec<(f64, f64)>, ()> {
        let mut points = Vec::<(f64, f64)>::new();
        let lines: Vec<&str> = s.split('\n').collect();
        let mut t = 0.0;

        for line in lines {
            if line.trim().is_empty() {
                continue;
            }
            let line: Vec<f64> = line
                .split(',')
                .map(|x| x.trim().parse::<f64>().unwrap())
                .collect();
            if line.len() != 3 {
                return Err(());
            }
            let t_start = t + line[0];
            let duration = line[1];
            let strength = line[2];
            t = t_start + duration;
            points.push((t_start, strength));
            points.push((t, 0.0));
        }
        Ok(points)
    }

    fn read_data_points(&self) -> Result<Vec<(f64, f64)>, ()> {
        let s = self.text_area.value();
        self.parse_data_points(&s)
    }
}
