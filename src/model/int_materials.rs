use std::collections::HashMap;
use byteable::Byteable;
use crate::model::int_units::*;

#[derive(Byteable, Debug, Clone, PartialEq)]
#[byteable(io_only)]
pub struct Material {
    pub name: String,
    pub density: UnitValue,
    pub modulus_youngs: UnitValue,
    pub modulus_shear: UnitValue
}
impl Material {
    pub fn default() -> Material { Material { 
        name: "None".to_string(), 
        density: UnitValue::kg_m3(0.0),
        modulus_youngs: UnitValue::pascals(0.0),
        modulus_shear: UnitValue::pascals(0.0),
    }}
}

pub fn materials_load() -> HashMap<String, Material> {
    let mut map: HashMap<String, Material> = HashMap::new();
    // Default materials.
    map.insert("Cardboard".to_owned(), Material { name: "Cardboard".to_owned(), density: UnitValue::kg_m3(679.887642), modulus_youngs: UnitValue::pascals(0.0), modulus_shear: UnitValue::pascals(0.0) });
    map.insert("Fiberglass".to_owned(), Material { name: "Fiberglass".to_owned(), density: UnitValue::kg_m3(1851.09358), modulus_youngs: UnitValue::pascals(0.0), modulus_shear: UnitValue::pascals(0.0) });
    map.insert("Carbon Fiber".to_owned(), Material { name: "Carbon Fiber".to_owned(), density: UnitValue::kg_m3(1781.89382), modulus_youngs: UnitValue::pascals(0.0), modulus_shear: UnitValue::pascals(0.0) });
    map.insert("PLA (100% Infill)".to_owned(), Material { name: "PLA (100% Infill)".to_owned(), density: UnitValue::kg_m3(1250.78566), modulus_youngs: UnitValue::pascals(0.0), modulus_shear: UnitValue::pascals(0.0) });
    return map;
}