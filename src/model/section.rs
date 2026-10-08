use core::f32;
use std::pin::pin;
use elegance::egui::{ Vec2, vec2 };

use enum_as_inner::EnumAsInner;
use crate::model::mass::MassProperties;
use crate::units::*;
use crate::materials::*;
use crate::model::curves::*;
use crate::model::finset::*;

/* -------------------------------------------------------------------------- */
/*                                  Sections                                  */
/* -------------------------------------------------------------------------- */
// The tubes, cones, and transitions that make up the body.
// These can all be described as an aft and fore diameter, constant thickness wall, and a length.
// Body tubes have the same aft and fore thickness, and nose cones have no fore diameter.
// They also have a "curve" variable which describes the shape of the curve which transitions between sections.
// They each have components that can be things like parachutes and fins.

pub struct RocketSection { 
    pub length: UnitValue, 
    pub diameter_fore: UnitValue, 
    pub diameter_aft: UnitValue, 
    pub wall_thickness: UnitValue,
    pub tapered: bool,
    
    pub curve_profile: CurveProfile,

    pub workbench_copy_diameter_fore: bool,
    pub workbench_copy_diameter_aft: bool,

    pub custom_material: bool,
    pub material: Material,

    pub components: Vec<RocketComponent>,

    pub int_wall_points: Vec<Vec2>,
    pub int_mass_properties: MassProperties
}

/* ----------------------------- Implementation ----------------------------- */
impl RocketSection {
    // Recalculates internal values that define the curve, mass, inertia, volume, etc.
    // Does nothing to true unit values or copied values.
    pub fn update_internal(&mut self) {
        // Sample curve along points and push these values as wall patches.
        self.int_wall_points.clear();
        const CURVE_POINTS: usize = 128;
        for i in 0..CURVE_POINTS {
            let x: f32 = (i as f32 / (CURVE_POINTS - 1) as f32) * self.length.value_true;
            let r: f32 = self.curve_profile.sample(self.diameter_fore.value_true / 2.0, self.diameter_aft.value_true / 2.0, self.length.value_true, x);
            self.int_wall_points.push(vec2(r, x));
        }

        // Compute mass and inertia by integrating wall patches as rectangular cross sections
        // revolved around the central axis.
        let mut volume: f32 = 0.0;
        let mut mass: f32 = 0.0;
        let mut ixx: f32 = 0.0;
        let mut iyy: f32 = 0.0;
        let mut cg: f32 = 0.0;
        // TODO Could use a trapezoidal integration for better accuracy.
        for i in 0..(self.int_wall_points.len() - 1) {
            let point0: &Vec2 = &self.int_wall_points[i+0];
            let point1: &Vec2 = &self.int_wall_points[i+1];

            let x_outer: f32 = (point0.x + point1.x) / 2.0;
            let x_inner: f32 = (x_outer - self.wall_thickness.value_true).max(0.0);
            let y_avg: f32 = (point1.y + point0.y) / 2.0;
            let height: f32 = point1.y - point0.y;

            let patch_volume: f32 = f32::consts::PI * (x_outer.powi(2) - x_inner.powi(2)) * height;
            let patch_mass: f32 = patch_volume * self.material.density.value_true;
            let patch_ixx: f32 = 0.5 * patch_mass * (x_outer.powi(2) + x_inner.powi(2));
            let patch_iyy: f32 = patch_mass * (((x_outer.powi(2) + x_inner.powi(2)) / 4.0) + (height * height / 12.0));

            volume += patch_volume;
            mass += patch_mass;
            ixx += patch_ixx;
            iyy += patch_iyy + patch_mass * y_avg * y_avg;
            cg += patch_mass * y_avg;
        }
        cg /= mass;
        iyy -= mass * cg * cg;

        // Apply computed values to section unitvalues.
        self.int_mass_properties.volume.value_true = volume; self.int_mass_properties.volume.update_ui();
        self.int_mass_properties.mass.value_true = mass; self.int_mass_properties.mass.update_ui();
        self.int_mass_properties.i_rotational.value_true = ixx; self.int_mass_properties.i_rotational.update_ui();
        self.int_mass_properties.i_longitudinal.value_true = iyy; self.int_mass_properties.i_longitudinal.update_ui();
        self.int_mass_properties.cg.value_true = cg; self.int_mass_properties.cg.update_ui();

        // TODO Update components.
    }
    
    // Top-view cross sectional area in square meters of a given spot in the cross section.
    // fn area_aft(&self) -> f32 { 0.25 * f32::consts::PI * self.diameter_aft.value_true.powi(2) }
    // fn area_fore(&self) -> f32 { 0.25 * f32::consts::PI * self.diameter_fore.value_true.powi(2) }
    // // Side-view cross sectional area in square meters.
    // fn area_planform(&self) -> f32 { 0.0 } // TODO
    // // Volume in cubic meters.
    // // Aero coefficients.
    // // (See OpenRocket technical docs)
    // // TODO No curve support.
    // // TODO Resolve coordinate systems.
    // fn cn(&self, alpha: f32, aref: f32, _lref: f32) -> f32 {
    //     let correction: f32 = if alpha == 0.0 { 1.0 } else { alpha.sin() / alpha };
    //     let cna_base: f32 = (2.0 / aref) * (self.area_aft() - self.area_fore()) * correction;
    //     let cn_base: f32 = alpha * cna_base;
    //     let k: f32 = 1.1;
    //     let cn_body_lift: f32 = (self.area_planform() / aref) * alpha.sin().powi(2);
    //     return cn_base + cn_body_lift;
    // }
    // fn cm(&self, alpha: f32, aref: f32, lref: f32) -> f32 {
    //     let correction: f32 = if alpha == 0.0 { 1.0 } else { alpha.sin() / alpha };
    //     return (2.0 / (aref * lref)) * (self.length*self.area_aft() - self.volume()) * correction;
    // }
    // fn cp(&self) -> f32 { 
    //     if self.area_fore() - self.area_aft() < 0.00001 { 0.0 }
    //     else { (self.length * self.area_aft() - self.volume()) / (self.area_fore() - self.area_aft()) }
    // }
    pub fn new() -> RocketSection { 
        let mut section: RocketSection = RocketSection { 
            length: UnitValue::inches(30.0), 
            diameter_fore: UnitValue::inches(3.0), 
            diameter_aft: UnitValue::inches(3.0), 
            wall_thickness: UnitValue::inches(0.08), 
            curve_profile: CurveProfile::Conical, 
            tapered: false, 
            workbench_copy_diameter_fore: false, 
            workbench_copy_diameter_aft: false,
            components: vec![],
            custom_material: false,
            material: Material::default(),
            
            int_wall_points: vec![],
            int_mass_properties: MassProperties::default()
        };
        section.update_internal();
        return section;
    }
}

/* -------------------------------------------------------------------------- */
/*                             Parachute Component                            */
/* -------------------------------------------------------------------------- */
// Mass component that also affects recovery descent rates.
#[derive(Debug)]
pub struct RocketComponentParachute {
    pub packed_diameter: UnitValue,
    pub packed_length: UnitValue,
    pub position_offset: UnitValue
}

/* -------------------------------------------------------------------------- */
/*                              Generic Component                             */
/* -------------------------------------------------------------------------- */
// Interface type for adressing rocket components as a generic type.
#[derive(Debug, EnumAsInner)]
pub enum RocketComponent {
    FinSet(RocketComponentFinSet),
    Parachute(RocketComponentParachute)
}