use elegance::{egui::{ CollapsingHeader, Ui, ComboBox }};
use crate::*;
use crate::units::*;
use crate::materials::*;
use crate::model::finset::*;
use crate::model::section::*;
use crate::workbench::ui_base::*;
use crate::workbench::ui_component::*;

// Section move/add/delete and subcomponent addition.
pub fn show_section_actions(ui: &mut Ui, section_idx: usize, app: &mut App) {
    ui.horizontal(|ui| {
        // Move/remove actions.
        let id_salt: String = "sectionmoveremove".to_owned() + &section_idx.to_string();
        let mut dummy_value: String = "#".to_owned();
        ComboBox::new(id_salt, "").selected_text("Move or Delete Section").show_ui(ui, |ui| {
            if section_idx < app.model.sections.len() - 1 {
                if ui.selectable_value(&mut dummy_value, "".to_string(), "Move Down").clicked() { 
                    app.last_workbench_action = WorkbenchAction::SwapSection(section_idx, section_idx + 1);
                }
            }
            if section_idx > 0 {
                if ui.selectable_value(&mut dummy_value, "".to_string(), "Move Up").clicked() { 
                    app.last_workbench_action = WorkbenchAction::SwapSection(section_idx, section_idx - 1);
                }
            }
            if ui.selectable_value(&mut dummy_value, "".to_string(), "Delete").clicked() { 
                app.last_workbench_action = WorkbenchAction::DeleteSection(section_idx);
            }
        });
        
        // Add subcomponent.
        let id_salt: String = "sectionaddcomponent".to_owned() + &section_idx.to_string();
        ComboBox::new(id_salt, "").selected_text("Add Component").show_ui(ui, |ui| {
            if ui.selectable_value(&mut dummy_value, "".to_string(), "Add Fin Set").clicked() { 
                app.model.sections[section_idx].components.push(RocketComponent::FinSet(RocketComponentFinSet::default()));
            }
            // TODO Parachute
            // if ui.selectable_value(&mut dummy_value, "".to_string(), "Add Fin Set").clicked() { 
            //     app.model.sections[section_idx].components.push(RocketComponent::FinSet(RocketComponentFinSet::default()));
            // }
        });
    });
}

// Section dimensions editor.
pub fn show_section_dimensions(ui: &mut Ui, section_idx: usize, app: &mut App) {
    let section: &mut RocketSection = &mut app.model.sections[section_idx];

    let id_salt: String = "sectiondimensions".to_owned() + &section_idx.to_string();
    CollapsingHeader::new(("Section Dimensions")).id_salt(id_salt).show(ui, |ui| {
        show_unit_value_field(ui, section_idx, UnitKind::Length, "Length".to_owned(),  &mut section.length, UnitRangeSpec::TubeLength, Some(&mut section.tapered), "Tapered");
        if !section.tapered {
            show_unit_value_field(ui, section_idx, UnitKind::Length,"Diameter".to_owned(),  &mut section.diameter_fore, UnitRangeSpec::TubeDiameter, Some(&mut section.copy_diameter_fore), "Copy from above");
        } else {
            show_unit_value_field(ui, section_idx, UnitKind::Length,"Fore Diameter".to_owned(),  &mut section.diameter_fore, UnitRangeSpec::TubeDiameter, Some(&mut section.copy_diameter_fore), "Copy from above");
            show_unit_value_field(ui, section_idx, UnitKind::Length,"Aft Diameter".to_owned(),  &mut section.diameter_aft, UnitRangeSpec::TubeDiameter, Some(&mut section.copy_diameter_aft), "Copy from above");
            show_curve_combo(ui, &mut section.curve_profile);
        }
        show_unit_value_field(ui, section_idx, UnitKind::Length, "Wall Thickness".to_owned(),  &mut section.wall_thickness, UnitRangeSpec::WallThickness, None, "");
        show_material_selector(ui, section_idx, &mut section.material, &mut section.custom_material, &app.materials);
    });
}

// Show all relevant section and component UIs.
pub fn show_section(ui: &mut Ui, section_idx: usize, app: &mut App) {
    // Determine label and id salt.
    let mut label: String = ". Body Tube".to_owned();
    if app.model.sections[section_idx].tapered { label = ". Transition".to_owned() };
    if app.model.sections[section_idx].tapered && app.model.sections[section_idx].diameter_fore.value_true ==  0.0 { label = ". Nose Cone".to_owned() };
    let id_salt: String = "sectionmaster".to_owned() + &section_idx.to_string();

    // Show collapsing header with dimensions, components, and then actions.
    CollapsingHeader::new((section_idx+1).to_string() + &label)
    .id_salt(id_salt)
    .show(ui, |ui| {
        show_section_dimensions(ui, section_idx, app);
        show_mass_properties(ui, section_idx, "Section Properties".to_string(), &mut app.model.sections[section_idx].int_mass_properties);
        for component_idx in 0..app.model.sections[section_idx].components.len() { show_component(ui, section_idx, component_idx, app); }
        show_section_actions(ui, section_idx, app);
    });
}