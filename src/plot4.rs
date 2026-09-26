use crate::SimulationState;
use egui_plot;

pub fn show(state_log: &Vec<SimulationState>, ui: &mut egui::Ui) {
    egui_plot::Plot::new("Plot 4")
    .legend(egui_plot::Legend::default())
    .show_axes(true)
    .show_grid(true)
    .width(625.0)
    .height(350.0)
    .show(ui, |plot_ui| {
        // plot_ui.line(egui_plot::Line::new("wb (x, deg/s)", egui_plot::PlotPoints::from_iter(state_log.iter().map(|d| [(*d).t as f64, (*d).wb.x.to_degrees() as f64]))));
        // plot_ui.line(egui_plot::Line::new("wb (y, deg/s)", egui_plot::PlotPoints::from_iter(state_log.iter().map(|d| [(*d).t as f64, (*d).wb.y.to_degrees() as f64]))));
        // plot_ui.line(egui_plot::Line::new("wb (z, deg/s)", egui_plot::PlotPoints::from_iter(state_log.iter().map(|d| [(*d).t as f64, (*d).wb.z.to_degrees() as f64]))));
        
        // plot_ui.line(egui_plot::Line::new("eul_rate (x, deg/s)", egui_plot::PlotPoints::from_iter(state_log.iter().map(|d| [(*d).t as f64, (*d).eul_rate.x.to_degrees() as f64]))));
        // plot_ui.line(egui_plot::Line::new("eul_rate (y, deg/s)", egui_plot::PlotPoints::from_iter(state_log.iter().map(|d| [(*d).t as f64, (*d).eul_rate.y.to_degrees() as f64]))));
        // plot_ui.line(egui_plot::Line::new("eul_rate (z, deg/s)", egui_plot::PlotPoints::from_iter(state_log.iter().map(|d| [(*d).t as f64, (*d).eul_rate.z.to_degrees() as f64]))));

        // plot_ui.line(egui_plot::Line::new("fb (x, lb)", egui_plot::PlotPoints::from_iter(state_log.iter().map(|d| [(*d).t as f64, (*d).fb.x * 0.2248 as f64]))));
        // plot_ui.line(egui_plot::Line::new("fb (y, lb)", egui_plot::PlotPoints::from_iter(state_log.iter().map(|d| [(*d).t as f64, (*d).fb.y * 0.2248 as f64]))));
        // plot_ui.line(egui_plot::Line::new("fb (z, lb)", egui_plot::PlotPoints::from_iter(state_log.iter().map(|d| [(*d).t as f64, (*d).fb.z * 0.2248 as f64]))));
        
        // plot_ui.line(egui_plot::Line::new("control alpha (rad/s^2)", egui_plot::PlotPoints::from_iter(state_log.iter().map(|d| [(*d).t as f64, (*d).control_alg_a_desired as f64]))));
        // plot_ui.line(egui_plot::Line::new("control m factor (rad/s^2)", egui_plot::PlotPoints::from_iter(state_log.iter().map(|d| [(*d).t as f64, (*d).control_alg_m_factor as f64]))));
        plot_ui.line(egui_plot::Line::new("control angle (deg)", egui_plot::PlotPoints::from_iter(state_log.iter().map(|d| [(*d).t as f64, (*d).control_alg_angle as f64]))));
        
        // plot_ui.line(egui_plot::Line::new("mb (x, Nm)", egui_plot::PlotPoints::from_iter(state_log.iter().map(|d| [(*d).t as f64, (*d).mb.x as f64]))));
        // plot_ui.line(egui_plot::Line::new("mb (y, Nm)", egui_plot::PlotPoints::from_iter(state_log.iter().map(|d| [(*d).t as f64, (*d).mb.y as f64]))));
        // plot_ui.line(egui_plot::Line::new("mb (z, Nm)", egui_plot::PlotPoints::from_iter(state_log.iter().map(|d| [(*d).t as f64, (*d).mb.z as f64]))));
        
        // plot_ui.line(egui_plot::Line::new("incidence (deg)", egui_plot::PlotPoints::from_iter(state_log.iter().map(|d| [(*d).t as f64, (*d).incidence.to_degrees() as f64]))));
        // plot_ui.line(egui_plot::Line::new("sideslip (deg)", egui_plot::PlotPoints::from_iter(state_log.iter().map(|d| [(*d).t as f64, (*d).sideslip.to_degrees() as f64]))));

        // plot_ui.line(egui_plot::Line::new("mach", egui_plot::PlotPoints::from_iter(state_log.iter().map(|d| [(*d).t as f64, (*d).mach as f64]))));

        // plot_ui.line(egui_plot::Line::new("fb (motor, N)", egui_plot::PlotPoints::from_iter(state_log.iter().map(|d| [(*d).t as f64, (*d).fb_thrust.x as f64]))));
        
        // plot_ui.line(egui_plot::Line::new("air rho (kg/m^3)", egui_plot::PlotPoints::from_iter(state_log.iter().map(|d| [(*d).t as f64, (*d).air_rho as f64]))));
        // plot_ui.line(egui_plot::Line::new("air a (m/s)", egui_plot::PlotPoints::from_iter(state_log.iter().map(|d| [(*d).t as f64, (*d).air_a as f64]))));
        // plot_ui.line(egui_plot::Line::new("air p (N/m^2)", egui_plot::PlotPoints::from_iter(state_log.iter().map(|d| [(*d).t as f64, (*d).air_p as f64]))));
        
        // plot_ui.line(egui_plot::Line::new("m (kg)", egui_plot::PlotPoints::from_iter(state_log.iter().map(|d| [(*d).t as f64, (*d).m as f64]))));
        // plot_ui.line(egui_plot::Line::new("cp (m)", egui_plot::PlotPoints::from_iter(state_log.iter().map(|d| [(*d).t as f64, (*d).cp as f64]))));
        // plot_ui.line(egui_plot::Line::new("cg (m)", egui_plot::PlotPoints::from_iter(state_log.iter().map(|d| [(*d).t as f64, (*d).cg as f64]))));
        // plot_ui.line(egui_plot::Line::new("Ixx (kg*m^2)", egui_plot::PlotPoints::from_iter(state_log.iter().map(|d| [(*d).t as f64, (*d).i.m11 as f64]))));
        // plot_ui.line(egui_plot::Line::new("Iyy (kg*m^2)", egui_plot::PlotPoints::from_iter(state_log.iter().map(|d| [(*d).t as f64, (*d).i.m22 as f64]))));
        // plot_ui.line(egui_plot::Line::new("Izz (kg*m^2)", egui_plot::PlotPoints::from_iter(state_log.iter().map(|d| [(*d).t as f64, (*d).i.m33 as f64]))));
        
        // plot_ui.line(egui_plot::Line::new("cd", egui_plot::PlotPoints::from_iter(state_log.iter().map(|d| [(*d).t as f64, (*d).cd as f64]))));
        // plot_ui.line(egui_plot::Line::new("cna", egui_plot::PlotPoints::from_iter(state_log.iter().map(|d| [(*d).t as f64, (*d).cna as f64]))));
        // plot_ui.line(egui_plot::Line::new("aref (m^2)", egui_plot::PlotPoints::from_iter(state_log.iter().map(|d| [(*d).t as f64, (*d).a_ref as f64]))));
        // plot_ui.line(egui_plot::Line::new("lref (m)", egui_plot::PlotPoints::from_iter(state_log.iter().map(|d| [(*d).t as f64, (*d).l_ref as f64]))));
        
        // plot_ui.line(egui_plot::Line::new("fb (airframe, x, N)", egui_plot::PlotPoints::from_iter(state_log.iter().map(|d| [(*d).t as f64, (*d).fb_airframe.x as f64]))));
        // plot_ui.line(egui_plot::Line::new("fb (airframe, y, N)", egui_plot::PlotPoints::from_iter(state_log.iter().map(|d| [(*d).t as f64, (*d).fb_airframe.y as f64]))));
        // plot_ui.line(egui_plot::Line::new("fb (airframe, z, N)", egui_plot::PlotPoints::from_iter(state_log.iter().map(|d| [(*d).t as f64, (*d).fb_airframe.z as f64]))));

        // plot_ui.line(egui_plot::Line::new("fb (controls, x, N)", egui_plot::PlotPoints::from_iter(state_log.iter().map(|d| [(*d).t as f64, (*d).fb_controls.x as f64]))));
        // plot_ui.line(egui_plot::Line::new("fb (controls, y, N)", egui_plot::PlotPoints::from_iter(state_log.iter().map(|d| [(*d).t as f64, (*d).fb_controls.y as f64]))));
        // plot_ui.line(egui_plot::Line::new("fb (controls, z, N)", egui_plot::PlotPoints::from_iter(state_log.iter().map(|d| [(*d).t as f64, (*d).fb_controls.z as f64]))));
        
        // plot_ui.line(egui_plot::Line::new("ae (x, m/s^2)", egui_plot::PlotPoints::from_iter(state_log.iter().map(|d| [(*d).t as f64, (*d).ae.x as f64]))));
        // plot_ui.line(egui_plot::Line::new("ae (y, m/s^2)", egui_plot::PlotPoints::from_iter(state_log.iter().map(|d| [(*d).t as f64, (*d).ae.y as f64]))));
        // plot_ui.line(egui_plot::Line::new("ae (z, m/s^2)", egui_plot::PlotPoints::from_iter(state_log.iter().map(|d| [(*d).t as f64, (*d).ae.z as f64]))));

        // plot_ui.line(egui_plot::Line::new("ve (x, m/s)", egui_plot::PlotPoints::from_iter(state_log.iter().map(|d| [(*d).t as f64, (*d).ve.x as f64]))));
        // plot_ui.line(egui_plot::Line::new("ve (y, m/s)", egui_plot::PlotPoints::from_iter(state_log.iter().map(|d| [(*d).t as f64, (*d).ve.y as f64]))));
        // plot_ui.line(egui_plot::Line::new("ve (z, m/s)", egui_plot::PlotPoints::from_iter(state_log.iter().map(|d| [(*d).t as f64, (*d).ve.z as f64]))));

        // plot_ui.line(egui_plot::Line::new("xe (x, ft)", egui_plot::PlotPoints::from_iter(state_log.iter().map(|d| [(*d).t as f64, (*d).xe.x / 0.3048 as f64]))));
        // plot_ui.line(egui_plot::Line::new("xe (y, ft)", egui_plot::PlotPoints::from_iter(state_log.iter().map(|d| [(*d).t as f64, (*d).xe.y / 0.3048 as f64]))));
        // plot_ui.line(egui_plot::Line::new("xe (z, ft)", egui_plot::PlotPoints::from_iter(state_log.iter().map(|d| [(*d).t as f64, (*d).xe.z / 0.3048 as f64]))));
    });
}