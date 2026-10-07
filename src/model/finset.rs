/* -------------------------------------------------------------------------- */
/*                                Fin Component                               */
/* -------------------------------------------------------------------------- */
use crate::{materials::Material, model::mass::MassProperties, units::*};

// Fins that attach to airframe sections as a component.
#[derive(Debug)]
pub struct RocketComponentFinSet {
    pub count: u32,
    pub chord_root: UnitValue,
    pub chord_tip: UnitValue,
    pub height: UnitValue,
    pub thickness: UnitValue,
    pub sweep: UnitValue,
    pub position_offset: UnitValue,
    pub material: Material,
    pub int_mass_properties: MassProperties
}
impl RocketComponentFinSet {
    //fn cn(&self, alpha: f32, aref: f32, lref: f32, body_diameter: f32) -> f32 {
    //    // TODO Supersonic behavior.
    //    let k: f32 = 1.0 + (body_diameter / 2.0) / (self.height + (body_diameter / 2.0));
    //    let lm = (self.chord_root + self.chord_tip) / 2.0;
    //    let cna = (2.0 * N * (self.height/body_diameter).powi(2) * k) / (1.0 + (1.0 + (2.0 * lm / body_diameter).powi(2)).sqrt());
    //    return cna * alpha;
    //}
    //fn cm(&self, alpha: f32, aref: f32, lref: f32, body_diameter: f32) -> f32 {
    //    // TODO.
    //    0.0
    //}
    //fn cp(&self, alpha: f32, aref: f32, lref: f32, body_diameter: f32) -> f32 {
    //    // TODO.
    //    0.0
    //}
    // Recalculates internal mass properties.
    pub fn update(&mut self) {
        // TODO What to do with cg and inertia...
        let single_fin_area = (self.chord_root.value_true + self.chord_tip.value_true) * self.height.value_true / 2.0;
        self.int_mass_properties.mass.value_true = single_fin_area * self.material.density.value_true * 4.0;
    }
    // Default values for a normal looking fin.
    pub fn default() -> RocketComponentFinSet { RocketComponentFinSet { 
        count: 4,
        chord_root: UnitValue::inches(4.0), 
        chord_tip: UnitValue::inches(2.0), 
        height: UnitValue::inches(2.5), 
        sweep: UnitValue::inches(2.5), 
        thickness: UnitValue::inches(0.12),
        position_offset: UnitValue::inches(0.0),
        material: Material::default(),
        int_mass_properties: MassProperties::default()
    }}
}