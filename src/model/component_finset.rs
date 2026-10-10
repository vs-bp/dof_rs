use std::f32;

use byteable::Byteable;
use egui::{Vec2, vec2};

/* -------------------------------------------------------------------------- */
/*                                Fin Component                               */
/* -------------------------------------------------------------------------- */
use crate::model::int_section::*;
use crate::model::int_units::*;
use crate::model::int_materials::*;
use crate::model::int_mass::*;

// Fins that attach to airframe sections as a component.
#[derive(Byteable, Debug)]
#[byteable(io_only)]
pub struct RocketComponentFinSet {
    pub count: u32,
    pub chord_root: UnitValue,
    pub chord_tip: UnitValue,
    pub height: UnitValue,
    pub thickness: UnitValue, // TODO.
    pub sweep: UnitValue,
    pub position_offset: UnitValue,
    pub material: Material, // TODO.
    pub int_mass_properties: MassProperties // TODO.
}
impl RocketComponentFinSet {    /* -------------------------------------------------------------------------- */
    /*                               Internal Values                              */
    /* -------------------------------------------------------------------------- */
    // Recalculates internal mass properties.
    pub fn update(&mut self) {
        // TODO What to do with cg and inertia...
        // TODO Unfinished.
        let single_fin_area = (self.chord_root.value_true + self.chord_tip.value_true) * self.height.value_true / 2.0;
        self.int_mass_properties.mass.value_true = single_fin_area * self.material.density.value_true * 4.0;

        // TODO Roll dynamics.
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

    /* -------------------------------------------------------------------------- */
    /*                                Aerodynamics                                */
    /* -------------------------------------------------------------------------- */
    fn cna_subsonic(&self, _alpha: f32, mach: f32, aref: f32, _lref: f32, body_radius: f32) -> f32 {
        // Fin shape variables.
        let span: f32 = self.height.value_true;
        let root: f32 = self.chord_root.value_true;
        let tip: f32 = self.chord_tip.value_true;
        let sweep: f32 = self.sweep.value_true;

        let area: f32 = (self.chord_root.value_true + self.chord_tip.value_true) * span / 2.0;
        let midchord_angle: f32 = ((sweep + (tip / 2.0) - (root / 2.0)) / span).atan();

        // Determine CNa 1 factoring in Diederich sweep correction.
        // (See OpenRocket technical documentation)
        let beta: f32 = (1.0 - mach * mach).sqrt();
        let top_term: f32 = 2.0 * f32::consts::PI * span * span / aref;
        let bottom_term: f32 = 1.0 + ((beta * span * span) / (area * midchord_angle.cos())).powi(2);
        let cna1: f32 = top_term / (1.0 + bottom_term.sqrt());

        // Account for interference due to fin count.
        let cnan: f32 = (self.count as f32 / 2.0) * cna1 * match self.count {
            // TODO Sum term does not simplify for N < 3.
            1 => 1.000,
            2 => 1.000,
            3 => 1.000,
            4 => 1.000,
            5 => 0.948,
            6 => 0.913,
            7 => 0.854,
            8 => 0.810,
            _ => 0.750
        };

        // Account for interference due to the rocket body.
        let ktb: f32 = 1.0 + (body_radius / (span + body_radius));
        let cnatb: f32 = cnan * ktb;
        return cnatb;

    }
    fn cna_supersonic(&self, alpha: f32, mach: f32, aref: f32, _lref: f32, body_radius: f32) -> f32 { 0.0 } // TODO.
    pub fn cna(&self, alpha: f32, mach: f32, aref: f32, lref: f32, body_radius: f32) -> f32 {
        if mach < 1.0 { self.cna_subsonic(alpha, mach, aref, lref, body_radius) }
        else { self.cna_supersonic(alpha, mach, aref, lref, body_radius) }
    }
    // TODO CP.

    /* -------------------------------------------------------------------------- */
    /*                                  Graphics                                  */
    /* -------------------------------------------------------------------------- */
    // Returns vector of lines made up of Vec2's that represent this component's lines.
    pub fn draw(&self, parent: &RocketSection) -> Vec<Vec<Vec2>> {
        let mut paths: Vec<Vec<Vec2>> = vec![];

        // Loop through all fins in the set.
        for i in 0..self.count {
            // Angle for given fin.
            // TODO Angle offset parameter.
            let angle_offset: f32 = 0.0;
            let angle: f32 = (i as f32 * 2.0 * f32::consts::PI / self.count as f32) + angle_offset;

            // Basic coordinates.
            let offset: f32 = parent.length.value_true;
            let body_diameter: f32 = parent.diameter_aft.value_true;
            let mut c_root_low: Vec2 = vec2(-body_diameter / 2.0, offset - self.position_offset.value_true);
            let mut c_root_high: Vec2 = c_root_low + vec2(0.0, -self.chord_root.value_true);
            let mut c_tip_high: Vec2 = vec2(
                -self.height.value_true - body_diameter / 2.0, 
                offset - self.position_offset.value_true - self.chord_root.value_true + self.sweep.value_true
            );
            let mut c_tip_low: Vec2 = c_tip_high + vec2(0.0, self.chord_tip.value_true);

            // Transform y axes by angle.
            c_root_low.x *= angle.cos();
            c_root_high.x *= angle.cos();
            c_tip_low.x *= angle.cos();
            c_tip_high.x *= angle.cos();

            // Add to output.
            paths.push(vec![c_root_low, c_tip_low, c_tip_high, c_root_high]);
        }
        return paths;
    }
}