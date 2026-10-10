use elegance::{egui::{ Ui }};
use crate::*;
use crate::editor::ui_base::*;

// Show fin set variant of component type.
pub fn show_component_fin_set(ui: &mut Ui, section_idx: usize, component_idx: usize, app: &mut App) {
    let component: &mut RocketComponentFinSet = app.model.sections[section_idx].components[component_idx].as_fin_set_mut().unwrap();
    CollapsingSection::new("componentdimensions".to_owned() + &(section_idx*1000 + component_idx).to_string(), "Component Dimensions").show(ui, |ui| {
        show_unit_value_field(ui, section_idx*1000 + component_idx, UnitKind::Length, "Root Chord".to_owned(),  &mut component.chord_root, UnitRangeSpec::FinRoot, None, "");
        show_unit_value_field(ui, section_idx*1000 + component_idx, UnitKind::Length, "Tip Chord".to_owned(),  &mut component.chord_tip, UnitRangeSpec::FinTip, None, "");
        show_unit_value_field(ui, section_idx*1000 + component_idx, UnitKind::Length, "Height".to_owned(),  &mut component.height, UnitRangeSpec::FinHeight, None, "");
        show_unit_value_field(ui, section_idx*1000 + component_idx, UnitKind::Length, "Sweep".to_owned(),  &mut component.sweep, UnitRangeSpec::FinSweep, None, "");
        show_unit_value_field(ui, section_idx*1000 + component_idx, UnitKind::Length, "Offset".to_owned(),  &mut component.position_offset, UnitRangeSpec::FinOffset, None, "");
    });

    CollapsingSection::new("componentmassproperties".to_owned() + &(section_idx*1000 + component_idx).to_string(), "Component Properties").show(ui, |ui| {
        show_mass_properties(ui, section_idx*1000 + component_idx, &mut component.int_mass_properties);
    });
}

// Show parachute variant of component type.
pub fn show_component_parachute(ui: &mut Ui, _section_idx: usize, _component_idx: usize, _app: &mut App) {
    ui.label("Hello!");
}

// Show generic mass variant of component type.
pub fn show_component_generic_mass(ui: &mut Ui, _section_idx: usize, _component_idx: usize, _app: &mut App) {
    ui.label("Hello!");
}

// Generic component show function that branches by variant.
pub fn show_component(ui: &mut Ui, section_idx: usize, component_idx: usize, app: &mut App) {
    match app.model.sections[section_idx].components[component_idx] {
        RocketComponent::FinSet(_) => show_component_fin_set(ui, section_idx, component_idx, app),
        RocketComponent::Parachute(_) => show_component_parachute(ui, section_idx, component_idx, app),
        RocketComponent::GenericMass(_) => show_component_generic_mass(ui, section_idx, component_idx, app)
    }
}