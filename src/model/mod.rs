pub mod int_units;
pub mod int_materials;
pub mod int_mass;
pub mod int_curves;
pub mod int_ui_base;
pub mod int_section;
pub mod int_motors;
pub mod component_fin_set;
pub mod component_generic_mass;
pub mod component_motor_mount;

use byteable::{Byteable, io::{ReadValue, WriteValue}};
use egui::Vec2;
use crate::model::{int_section::*, int_ui_base::ViewerLine};
use crate::model::int_mass::*;

#[derive(Byteable)]
#[byteable(io_only)]
pub struct RocketModel {
    pub sections: Vec<RocketSection>,
    pub int_mass_properties: MassProperties
}
impl RocketModel {
    /* ---------------------------- Aero Coefficients --------------------------- */
    pub fn cna(&self, alpha: f32, mach: f32) -> f32 {
        // Early exit if no sections.
        if self.sections.len() == 0 { return 0.0; }
        
        // Determine references.
        let lref: f32 = self.sections[0].diameter_aft.value_true;
        let aref: f32 = self.sections[0].int_area_aft.value_true;
        if lref == 0.0 { return 0.0 };
        if aref == 0.0 { return 0.0 };

        // Sum CNa's
        let mut cna_total: f32 = 0.0;
        for section in &self.sections {
            cna_total += section.cna(alpha, mach, aref, lref);
        }
        return cna_total;
    }
    pub fn cp(&self, alpha: f32, mach: f32) -> f32 {
        // Early exit if no sections.
        if self.sections.len() == 0 { return 0.0; }
        
        // Determine references.
        let lref: f32 = self.sections[0].diameter_aft.value_true;
        let aref: f32 = self.sections[0].int_area_aft.value_true;
        if lref == 0.0 { return 0.0 };
        if aref == 0.0 { return 0.0 };

        // Sum and combine CP's from sections and fins.
        // TODO Pending other aero components.
        let mut cp_total: f32 = 0.0;
        let mut cna_total: f32 = 0.0;
        let mut y0: f32 = 0.0;
        for section in &self.sections {
            // Section CP / CNA
            cp_total += (section.cp(alpha, mach, aref, lref) + y0) * section.cna(alpha, mach, aref, lref);
            cna_total += section.cna(alpha, mach, aref, lref);

            // Components CP / CNA
            for generic_component in &section.components { match generic_component {
                RocketComponent::FinSet(component) => {
                    cp_total += (component.cp(alpha, mach, aref, lref, section) + y0) * component.cna(alpha, mach, aref, lref, section);
                    cna_total += component.cna(alpha, mach, aref, lref, section);
                },
                _ => {}
            }}

            y0 += section.length.value_true;
        }
        if cna_total == 0.0 { return 0.0; }
        return cp_total / cna_total;
    }

    /* ---------------------------- Binary Save/Load ---------------------------- */
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.write_value(self).unwrap();
        return buf;
    }
    pub fn from_bytes(bytes: Vec<u8>) -> RocketModel {
        let mut buf: &[u8] = &bytes;
        return buf.read_value().unwrap();
    }

    /* ---------------- Internal Value Handling / Initialization ---------------- */
    pub fn default() -> RocketModel { RocketModel {
        sections: vec![],
        int_mass_properties: MassProperties::default()
    }}
    // Updates internal variables for whole model as well as all sections and components.
    pub fn update_internal(&mut self) {
        // Update all sections first.
        // This will also update their components.
        for section in &mut self.sections { section.update_internal(); }

        // Update mass properties from sections.
        let mut parts: Vec<(f32, MassProperties)> = vec![];
        let mut y_position: f32 = 0.0;
        for section in &mut self.sections {
            parts.push((y_position, section.int_mass_properties.clone()));
            y_position += section.length.value_true;
        }
        self.int_mass_properties = MassProperties::combine(Some(self.int_mass_properties.clone()), &parts);
    }

    /* -------------------------------------------------------------------------- */
    /*                                  Graphics                                  */
    /* -------------------------------------------------------------------------- */
    // Returns list of all lines that make up the model in the viewer.
    pub fn draw(&self) -> Vec<ViewerLine> {
        let mut draw_position: f32 = 0.0;
        let mut output: Vec<ViewerLine> = vec![];
        for section in &self.sections {
            // Section lines are offset by draw position which increments
            // by section length for each section that is finished.
            let mut section_lines: Vec<ViewerLine> = section.draw();
            for line in &mut section_lines {
                match line {
                    ViewerLine::Solid(_, _, path) => {
                        for point in path { point.y += draw_position; }
                    },
                    ViewerLine::Dashed(_, _, path) => {
                        for point in path { point.y += draw_position; }
                    }
                }
            }
            output.append(&mut section_lines);
            draw_position += section.length.value_true;
        }
        return output;
    }
}