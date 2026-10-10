use std::collections::HashMap;

use eframe::egui;
use elegance::*;
use crate::model::*;
use crate::model::int_units::*;
use crate::model::int_materials::*;
use crate::model::component_finset::*;
use crate::model::int_section::*;

mod model;
mod editor;

// Action enum used for propogating events that affect the order of section or component vectors.
pub enum WorkbenchAction {
    None,
    SwapSection(usize,usize),
    SwapComponent(usize,usize,usize),
    DeleteSection(usize),
    DeleteComponent(usize,usize)
}

// Possible references to selected sections and components for use in the UI display logic,
// indexed using their section or component indices.
pub enum WorkbenchSelection {
    None,
    Section(usize),
    Component(usize, usize)
}

// Full app state collection
struct App {
    workbench_selection: WorkbenchSelection,
    materials: HashMap<String, Material>,
    model: RocketModel
}

pub fn main() -> eframe::Result {
    let mut app: App = App {
        workbench_selection: WorkbenchSelection::None,
        materials: materials_load(),
        model: RocketModel::default()
    };
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_maximized(true).with_fullscreen(false),
        ..Default::default()
    };
    eframe::run_ui_native("dof_rs", native_options, move |ui, _frame| {
        Theme::charcoal().install(ui.ctx());
        egui::Panel::right("rightpanel").default_size(300.0).show(ui, |ui| {
            editor::model_viewer::show(ui, &mut app);
        });
        egui::Panel::left("leftpanel").default_size(600.0).show(ui, |ui| {
            editor::model_workbench::update(&mut app);
            editor::model_workbench::show(ui, &mut app);
            editor::model_analyzer::show(ui, &mut app);
        });
        egui::CentralPanel::default().show(ui,|ui| {
            editor::model_tree::show(ui, &mut app);
        });
    })
}