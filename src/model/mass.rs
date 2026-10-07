use nalgebra::Unit;

use crate::units::UnitValue;

#[derive(Clone, Debug)]
pub struct MassProperties {
    // Origin is assumed to be at some x-value but y=0 and z=0 (on the centerline of the rocket)
    // for cg and inertia calcs.
    pub volume: UnitValue,
    pub mass: UnitValue,
    pub cg: UnitValue,
    pub i_rotational: UnitValue,
    pub i_longitudinal: UnitValue
}
impl MassProperties {
    // Values all default to zero.
    pub fn default() -> MassProperties { MassProperties { 
        volume: UnitValue::m3(0.0), 
        mass: UnitValue::kg(0.0), 
        cg: UnitValue::meters(0.0), 
        i_rotational: UnitValue::kgm2(0.0), 
        i_longitudinal: UnitValue::kgm2(0.0) 
    }}
    // Combine multiple mass properties given their relative x-axis positions in a tuple.
    // Template specifies output units, default if none.
    pub fn combine(template: Option<MassProperties>, parts: &Vec<(f32, MassProperties)>) -> MassProperties {
        // Zero out template values for use.
        let mut output: MassProperties = template.unwrap_or(MassProperties::default());
        output.cg.value_true = 0.0;
        output.i_longitudinal.value_true = 0.0;
        output.i_rotational.value_true = 0.0;
        output.mass.value_true = 0.0;
        output.volume.value_true = 0.0;

        let mut volume: f32 = 0.0;
        let mut mass: f32 = 0.0;
        let mut cg: f32 = 0.0;
        let mut ixx: f32 = 0.0;
        let mut iyy: f32 = 0.0;
        let mut y_position: f32 = 0.0;
        for part in parts {
            y_position = part.0;
            volume += part.1.volume.value_true;
            mass += part.1.mass.value_true;
            cg += part.1.mass.value_true * (part.1.cg.value_true + y_position);
            ixx += part.1.i_rotational.value_true;
            iyy += part.1.i_longitudinal.value_true + part.1.mass.value_true * ((part.1.cg.value_true + y_position).powi(2));
        }
        cg /= mass;
        iyy -= mass * (cg * cg);

        output.volume.value_true = volume; output.volume.update_ui();
        output.mass.value_true = mass; output.mass.update_ui();
        output.cg.value_true = cg; output.cg.update_ui();
        output.i_rotational.value_true = ixx; output.i_rotational.update_ui();
        output.i_longitudinal.value_true = iyy; output.i_longitudinal.update_ui();
        return output;
    }
}