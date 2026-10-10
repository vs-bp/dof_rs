use byteable::Byteable;
use egui::{Vec2, vec2};
use crate::model::int_section::*;
use crate::model::int_units::*;

/* -------------------------------------------------------------------------- */
/*                             Parachute Component                            */
/* -------------------------------------------------------------------------- */
// Mass component that also affects recovery descent rates.

#[derive(Byteable, Debug)]
#[byteable(io_only)]
pub struct RocketComponentParachute {
    pub packed_diameter: UnitValue,
    pub packed_length: UnitValue,
    pub position_offset: UnitValue
}
impl RocketComponentParachute {
    /* -------------------------------------------------------------------------- */
    /*                                  Graphics                                  */
    /* -------------------------------------------------------------------------- */
    // Returns vector of lines made up of Vec2's that represent this component's lines.
    pub fn draw(&self, parent: &RocketSection) -> Vec<Vec<Vec2>> {
        // TODO Variable color?

        // Basic coordinates.
        let offset: f32 = parent.length.value_true;
        let c1: Vec2 = vec2(-self.packed_diameter.value_true / 2.0, -self.position_offset.value_true + offset);
        let c2: Vec2 = vec2(self.packed_diameter.value_true / 2.0, -self.position_offset.value_true + offset);
        let c3: Vec2 = vec2(self.packed_diameter.value_true / 2.0, -self.position_offset.value_true + offset - self.packed_length.value_true);
        let c4: Vec2 = vec2(-self.packed_diameter.value_true / 2.0, -self.position_offset.value_true + offset - self.packed_length.value_true);

        // Output.
        return vec![vec![c1,c2,c3,c4,c1]];
    }
}