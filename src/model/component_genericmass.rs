use byteable::Byteable;
use egui::Vec2;
use crate::model::int_section::*;
use crate::model::int_units::*;

/* -------------------------------------------------------------------------- */
/*                           Generic Mass Component                           */
/* -------------------------------------------------------------------------- */
// Generic mass component.

#[derive(Byteable, Debug)]
#[byteable(io_only)]
pub struct RocketComponentGenericMass {
    pub position_offset: UnitValue
}
impl RocketComponentGenericMass {
    /* -------------------------------------------------------------------------- */
    /*                                  Graphics                                  */
    /* -------------------------------------------------------------------------- */
    // Returns vector of lines made up of Vec2's that represent this component's lines.
    pub fn draw(&self, parent: &RocketSection) -> Vec<Vec<Vec2>> {
        // TODO.
        return vec![];
        /*
        // Basic coordinates.
        let mut c_root_low: Vec2 = vec2(-body_diameter / 2.0, draw_position - component.position_offset.value_true);
        let mut c_root_high: Vec2 = c_root_low + vec2(0.0, -component.chord_root.value_true);
        let mut c_tip_high: Vec2 = vec2(
            -component.height.value_true - body_diameter / 2.0, 
            draw_position - component.position_offset.value_true - component.chord_root.value_true + component.sweep.value_true
        );
        let mut c_tip_low: Vec2 = c_tip_high + vec2(0.0, component.chord_tip.value_true);

        // Transform y axes by angle.
        c_root_low.x *= angle.cos();
        c_root_high.x *= angle.cos();
        c_tip_low.x *= angle.cos();
        c_tip_high.x *= angle.cos();

        // Output.
        return vec![vec![c_root_low, c_tip_low, c_tip_high, c_root_high]];
        */
    }
}