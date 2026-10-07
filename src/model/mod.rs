pub mod mass;
pub mod curves;
pub mod finset;
pub mod section;

use crate::{model::{mass::MassProperties, section::*}, units::UnitValue};

pub struct RocketModel {
    pub sections: Vec<RocketSection>,
    pub int_mass_properties: MassProperties
    // TODO Area
}
impl RocketModel {
    // Default values are zero on everything and no sections.
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
}