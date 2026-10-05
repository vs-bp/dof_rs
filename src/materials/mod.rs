use std::collections::HashMap;

use crate::units::UnitValue;

#[derive(Clone, PartialEq)]
pub struct MaterialSpec {
    pub name: String,
    pub density: UnitValue,
    pub modulus_youngs: UnitValue,
    pub modulus_shear: UnitValue
}
impl MaterialSpec {
    pub fn default() -> MaterialSpec { MaterialSpec { 
        name: "None".to_string(), 
        density: UnitValue::kg_m3(0.0),
        modulus_youngs: UnitValue::pascals(0.0),
        modulus_shear: UnitValue::pascals(0.0),
    }}
}

pub fn materials_load() -> HashMap<String, MaterialSpec> {
    let mut map: HashMap<String, MaterialSpec> = HashMap::new();
    // Default materials.
    map.insert("Cardboard".to_owned(), MaterialSpec { name: "Cardboard".to_owned(), density: UnitValue::kg_m3(679.887642), modulus_youngs: UnitValue::pascals(0.0), modulus_shear: UnitValue::pascals(0.0) });
    map.insert("Fiberglass".to_owned(), MaterialSpec { name: "Fiberglass".to_owned(), density: UnitValue::kg_m3(1851.09358), modulus_youngs: UnitValue::pascals(0.0), modulus_shear: UnitValue::pascals(0.0) });
    map.insert("Carbon Fiber".to_owned(), MaterialSpec { name: "Carbon Fiber".to_owned(), density: UnitValue::kg_m3(1781.89382), modulus_youngs: UnitValue::pascals(0.0), modulus_shear: UnitValue::pascals(0.0) });
    map.insert("PLA (100% Infill)".to_owned(), MaterialSpec { name: "PLA (100% Infill)".to_owned(), density: UnitValue::kg_m3(1250.78566), modulus_youngs: UnitValue::pascals(0.0), modulus_shear: UnitValue::pascals(0.0) });
    return map;
}