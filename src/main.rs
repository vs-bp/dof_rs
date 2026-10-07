use std::collections::HashMap;

use eframe::egui;
use elegance::*;
use crate::model::RocketModel;
use crate::units::*;
use crate::materials::*;
use crate::model::finset::*;
use crate::model::section::*;

mod units;
mod materials;
mod model;
mod workbench;
mod viewer;

// Action enum used for propogating events that affect the order of section or component vectors.
pub enum WorkbenchAction {
    None,
    SwapSection(usize,usize),
    SwapComponent(usize,usize,usize),
    DeleteSection(usize),
    DeleteComponent(usize,usize)
}

// Full app state collection
struct App {
    last_workbench_action: WorkbenchAction,
    materials: HashMap<String, Material>,
    model: RocketModel
}

pub fn main() -> eframe::Result {
    let mut app: App = App {
        last_workbench_action: WorkbenchAction::None,
        materials: materials_load(),
        model: RocketModel::default()
    };
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_maximized(true).with_fullscreen(false),
        ..Default::default()
    };
    eframe::run_ui_native("dof_rs", native_options, move |ui, frame| {
        Theme::slate().install(ui.ctx());
        egui::Panel::right("rightpanel").default_size(450.0).show(ui, |ui| {
            viewer::show(ui, &mut app);
        });
        egui::CentralPanel::default().show(ui,|ui| {
            workbench::update(&mut app);
            workbench::show(ui, &mut app);
        });
    })
}