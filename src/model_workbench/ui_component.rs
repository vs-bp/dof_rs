use elegance::{egui::{ CollapsingHeader, Ui, ComboBox }};
use crate::*;
use crate::units::*;
use crate::materials::*;
use crate::model::finset::*;
use crate::model::section::*;
use crate::model_workbench::ui_base::*;

// Show fin set variant of component type.
pub fn show_component_fin_set(ui: &mut Ui, section_idx: usize, component_idx: usize, app: &mut App) {
    let component: &mut RocketComponentFinSet = app.model.sections[section_idx].components[component_idx].as_fin_set_mut().unwrap();
    ui.heading("Dimensions");
    show_unit_value_field(ui, section_idx*1000 + component_idx, UnitKind::Length, "Root Chord".to_owned(),  &mut component.chord_root, UnitRangeSpec::FinRoot, None, "");
    show_unit_value_field(ui, section_idx*1000 + component_idx, UnitKind::Length, "Tip Chord".to_owned(),  &mut component.chord_tip, UnitRangeSpec::FinTip, None, "");
    show_unit_value_field(ui, section_idx*1000 + component_idx, UnitKind::Length, "Height".to_owned(),  &mut component.height, UnitRangeSpec::FinHeight, None, "");
    show_unit_value_field(ui, section_idx*1000 + component_idx, UnitKind::Length, "Sweep".to_owned(),  &mut component.sweep, UnitRangeSpec::FinSweep, None, "");
    show_unit_value_field(ui, section_idx*1000 + component_idx, UnitKind::Length, "Offset".to_owned(),  &mut component.position_offset, UnitRangeSpec::FinOffset, None, "");

    ui.separator();
    ui.heading("Mass Properties");
    show_mass_properties(ui, section_idx*1000 + component_idx, &mut component.int_mass_properties);
}

// Show parachute variant of component type.
pub fn show_component_parachute(ui: &mut Ui, section_idx: usize, component_idx: usize, app: &mut App) {
    ui.label("Hello!");
}

// Generic component show function that branches by variant.
pub fn show_component(ui: &mut Ui, section_idx: usize, component_idx: usize, app: &mut App) {
    match app.model.sections[section_idx].components[component_idx] {
        RocketComponent::FinSet(_) => show_component_fin_set(ui, section_idx, component_idx, app),
        RocketComponent::Parachute(_) => show_component_parachute(ui, section_idx, component_idx, app)
    }
}