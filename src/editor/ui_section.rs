use elegance::{egui::{ Ui }};
use crate::*;
use crate::editor::ui_base::*;

// Show all relevant section and component UIs.
pub fn show_section(ui: &mut Ui, section_idx: usize, app: &mut App) {    
    let section: &mut RocketSection = &mut app.model.sections[section_idx];

    CollapsingSection::new("sectiondimensions".to_owned() + &section_idx.to_string(), "Section Dimensions").show(ui, |ui| {
        show_unit_value_field(ui, section_idx, UnitKind::Length, "Length".to_owned(),  &mut section.length, UnitRangeSpec::TubeLength, Some(&mut section.tapered), "Tapered");
        if !section.tapered {
            show_unit_value_field(ui, section_idx, UnitKind::Length,"Diameter".to_owned(),  &mut section.diameter_fore, UnitRangeSpec::TubeDiameter, Some(&mut section.workbench_copy_diameter_fore), "Copy from above");
        } else {
            show_unit_value_field(ui, section_idx, UnitKind::Length,"Fore Diameter".to_owned(),  &mut section.diameter_fore, UnitRangeSpec::TubeDiameter, Some(&mut section.workbench_copy_diameter_fore), "Copy from above");
            show_unit_value_field(ui, section_idx, UnitKind::Length,"Aft Diameter".to_owned(),  &mut section.diameter_aft, UnitRangeSpec::TubeDiameter, Some(&mut section.workbench_copy_diameter_aft), "Copy from above");
            show_curve_combo(ui, &mut section.curve_profile);
        }
        show_unit_value_field(ui, section_idx, UnitKind::Length, "Wall Thickness".to_owned(),  &mut section.wall_thickness, UnitRangeSpec::WallThickness, None, "");
    });

    CollapsingSection::new("sectionproperties".to_owned() + &section_idx.to_string(), "Section Properties").show(ui, |ui| {
        show_material_selector(ui, section_idx, &mut section.material, &mut section.custom_material, &app.materials);
        show_mass_properties(ui, section_idx,  &mut section.int_mass_properties);
        show_unit_value_label(ui, section_idx, UnitKind::Area, "Fore Area".to_owned(), &mut section.int_area_fore);
        show_unit_value_label(ui, section_idx, UnitKind::Area, "Aft Area".to_owned(), &mut section.int_area_aft);
        show_unit_value_label(ui, section_idx, UnitKind::Area, "Planform Area".to_owned(), &mut section.int_area_planform);
        show_unit_value_label(ui, section_idx, UnitKind::Length, "Planform Area Centroid X".to_owned(), &mut section.int_area_planform_centroid);
        show_unit_value_label(ui, section_idx, UnitKind::Volume, "Aerodynamic Volume".to_owned(), &mut section.int_aero_volume);
    });
}