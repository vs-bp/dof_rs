use elegance::{egui::{ CollapsingHeader, Ui, ComboBox }};
use crate::*;
use crate::units::*;
use crate::materials::*;
use crate::model::finset::*;
use crate::model::section::*;
use crate::model_workbench::ui_base::*;
use crate::model_workbench::ui_component::*;

// Show all relevant section and component UIs.
pub fn show_section(ui: &mut Ui, section_idx: usize, app: &mut App) {    
    let section: &mut RocketSection = &mut app.model.sections[section_idx];

    ui.heading("Dimensions");
    show_unit_value_field(ui, section_idx, UnitKind::Length, "Length".to_owned(),  &mut section.length, UnitRangeSpec::TubeLength, Some(&mut section.tapered), "Tapered");
    if !section.tapered {
        show_unit_value_field(ui, section_idx, UnitKind::Length,"Diameter".to_owned(),  &mut section.diameter_fore, UnitRangeSpec::TubeDiameter, Some(&mut section.workbench_copy_diameter_fore), "Copy from above");
    } else {
        show_unit_value_field(ui, section_idx, UnitKind::Length,"Fore Diameter".to_owned(),  &mut section.diameter_fore, UnitRangeSpec::TubeDiameter, Some(&mut section.workbench_copy_diameter_fore), "Copy from above");
        show_unit_value_field(ui, section_idx, UnitKind::Length,"Aft Diameter".to_owned(),  &mut section.diameter_aft, UnitRangeSpec::TubeDiameter, Some(&mut section.workbench_copy_diameter_aft), "Copy from above");
        show_curve_combo(ui, &mut section.curve_profile);
    }
    show_unit_value_field(ui, section_idx, UnitKind::Length, "Wall Thickness".to_owned(),  &mut section.wall_thickness, UnitRangeSpec::WallThickness, None, "");
    
    ui.separator();
    ui.heading("Mass Properties");
    show_material_selector(ui, section_idx, &mut section.material, &mut section.custom_material, &app.materials);
    show_mass_properties(ui, section_idx,  &mut app.model.sections[section_idx].int_mass_properties);
}