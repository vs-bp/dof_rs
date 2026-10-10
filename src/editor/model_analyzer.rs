use elegance::egui::{ Ui };
use crate::*;
use egui_plot;

pub fn show(ui: &mut Ui, app: &mut App) {
    let mach: f32 = 0.0;
    let aoa_min: f32 = 0.0;
    let aoa_max: f32 = 10.0;

    const POINT_COUNT: usize = 128;
    let mut points_cna: Vec<[f64; 2]> = vec![];
    let mut points_cma: Vec<[f64; 2]> = vec![];
    let mut points_cp: Vec<[f64; 2]> = vec![];
    for i in 0..POINT_COUNT {
        let aoa: f32 = (i as f32 / (POINT_COUNT as f32 - 1.0)) * (aoa_max - aoa_min) + aoa_min;
        points_cna.push((aoa as f64, app.model.cna(aoa.to_radians(), mach) as f64).into());
        points_cma.push((aoa as f64, app.model.cma(aoa.to_radians(), mach) as f64).into());
        points_cp.push((aoa as f64, app.model.cp(aoa.to_radians(), mach) as f64 * 39.37008).into());
    }

    CollapsingSection::new("modelaero", "Model Aerodynamics").show(ui, |ui| {
        egui_plot::Plot::new("Model Aerodynamics")
        .legend(egui_plot::Legend::default())
        .show_axes(true)
        .show_grid(true)
        .height(300.0)
        .show(ui, |plot_ui| {
            plot_ui.line(egui_plot::Line::new("CNa vs Angle (deg)", egui_plot::PlotPoints::from_iter(points_cna)));
            plot_ui.line(egui_plot::Line::new("CMa vs Angle (deg)", egui_plot::PlotPoints::from_iter(points_cma)));
            plot_ui.line(egui_plot::Line::new("CP (in) vs Angle (deg)", egui_plot::PlotPoints::from_iter(points_cp)));
        });
    });
}