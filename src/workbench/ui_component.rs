use elegance::{egui::{ CollapsingHeader, Ui, ComboBox }};
use crate::*;
use crate::units::*;
use crate::materials::*;
use crate::model::finset::*;
use crate::model::section::*;
use crate::workbench::ui_base::*;

// Show fin set variant of component type.
pub fn show_component_fin_set(ui: &mut Ui, section_idx: usize, component_idx: usize, app: &mut App) {
    let component: &mut RocketComponentFinSet = app.model.sections[section_idx].components[component_idx].as_fin_set_mut().unwrap();
    show_unit_value_field(ui, section_idx*1000 + component_idx, UnitKind::Length, "Root Chord".to_owned(),  &mut component.chord_root, UnitRangeSpec::FinRoot, None, "");
    show_unit_value_field(ui, section_idx*1000 + component_idx, UnitKind::Length, "Tip Chord".to_owned(),  &mut component.chord_tip, UnitRangeSpec::FinTip, None, "");
    show_unit_value_field(ui, section_idx*1000 + component_idx, UnitKind::Length, "Height".to_owned(),  &mut component.height, UnitRangeSpec::FinHeight, None, "");
    show_unit_value_field(ui, section_idx*1000 + component_idx, UnitKind::Length, "Sweep".to_owned(),  &mut component.sweep, UnitRangeSpec::FinSweep, None, "");
    show_unit_value_field(ui, section_idx*1000 + component_idx, UnitKind::Length, "Offset".to_owned(),  &mut component.position_offset, UnitRangeSpec::FinOffset, None, "");
}

// Show parachute variant of component type.
pub fn show_component_parachute(ui: &mut Ui, section_idx: usize, component_idx: usize, app: &mut App) {
    ui.label("Hello!");
}

// Generic component show function that branches by variant.
pub fn show_component(ui: &mut Ui, section_idx: usize, component_idx: usize, app: &mut App) {
    let label = (section_idx + 1).to_string() + "." + &(component_idx + 1).to_string() + match app.model.sections[section_idx].components[component_idx] {
        RocketComponent::FinSet(_) => { ". Fin Set" },
        RocketComponent::Parachute(_) => { ". Parachute" }
    };
    let id_salt: String = "componentmaster".to_owned() + &(section_idx * 1000 + component_idx).to_string();
    CollapsingHeader::new(label).id_salt(id_salt).show(ui, |ui| {
        // Major dimensions by variant.
        match app.model.sections[section_idx].components[component_idx] {
            RocketComponent::FinSet(_) => show_component_fin_set(ui, section_idx, component_idx, app),
            RocketComponent::Parachute(_) => show_component_parachute(ui, section_idx, component_idx, app)
        }
        // Delete and move actions are generic for all components.
        let id_salt: String = "componentmoveremove".to_owned() + &(section_idx * 1000 + component_idx).to_string();
        let mut dummy_value: String = "#".to_owned();
        ComboBox::new(id_salt, "").selected_text("Move or Delete Component").show_ui(ui, |ui| {
            if component_idx < app.model.sections[section_idx].components.len() - 1 {
                if ui.selectable_value(&mut dummy_value, "".to_string(), "Move Down").clicked() { 
                    app.last_workbench_action = WorkbenchAction::SwapComponent(section_idx, component_idx, component_idx + 1);
                }
            }
            if component_idx > 0 {
                if ui.selectable_value(&mut dummy_value, "".to_string(), "Move Up").clicked() { 
                    app.last_workbench_action = WorkbenchAction::SwapComponent(section_idx, component_idx, component_idx - 1);
                }
            }
            if ui.selectable_value(&mut dummy_value, "".to_string(), "Delete").clicked() { 
                app.last_workbench_action = WorkbenchAction::DeleteComponent(section_idx, component_idx);
            }
        });
    });
}