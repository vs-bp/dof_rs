use core::f32;
use std::pin::pin;
use byteable::Byteable;
use elegance::egui::{ Vec2, vec2 };

use enum_as_inner::EnumAsInner;
use crate::model::int_mass::MassProperties;
use crate::model::int_units::*;
use crate::model::int_materials::*;
use crate::model::int_curves::*;
use crate::model::component_finset::*;
use crate::model::component_genericmass::*;
use crate::model::component_parachute::*;

/* -------------------------------------------------------------------------- */
/*                                  Sections                                  */
/* -------------------------------------------------------------------------- */
// The tubes, cones, and transitions that make up the body.
// These can all be described as an aft and fore diameter, constant thickness wall, and a length.
// Body tubes have the same aft and fore thickness, and nose cones have no fore diameter.
// They also have a "curve" variable which describes the shape of the curve which transitions between sections.
// They each have components that can be things like parachutes and fins.

#[derive(Byteable)]
#[byteable(io_only)]
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

    pub int_wall_points: Vec<(f32, f32)>,
    pub int_mass_properties: MassProperties,
    pub int_area_aft: UnitValue,
    pub int_area_fore: UnitValue,
    pub int_area_planform: UnitValue,
    pub int_area_planform_centroid: UnitValue,
    pub int_aero_volume: UnitValue
    // TODO Display this?
}

/* ----------------------------- Implementation ----------------------------- */
impl RocketSection {
    /* -------------------------------------------------------------------------- */
    /*                             Internal Variables                             */
    /* -------------------------------------------------------------------------- */
    // Recalculates internal values that define the curve, mass, inertia, volume, etc.
    // Does nothing to true unit values or copied values.
    pub fn update_internal(&mut self) {
        // Sample curve along points and push these values as wall patches.
        self.int_wall_points.clear();
        const CURVE_POINTS: usize = 128;
        for i in 0..CURVE_POINTS {
            let x: f32 = (i as f32 / (CURVE_POINTS - 1) as f32) * self.length.value_true;
            let r: f32 = self.curve_profile.sample(self.diameter_fore.value_true / 2.0, self.diameter_aft.value_true / 2.0, self.length.value_true, x);
            self.int_wall_points.push((r, x));
        }

        // Compute mass and inertia by integrating wall patches as rectangular cross sections
        // revolved around the central axis.
        let mut material_volume: f32 = 0.0;
        let mut mass: f32 = 0.0;
        let mut ixx: f32 = 0.0;
        let mut iyy: f32 = 0.0;
        let mut cg: f32 = 0.0;
        let mut planform: f32 = 0.0;
        let mut planform_centroid: f32 = 0.0;
        let mut aero_volume: f32 = 0.0;
        // TODO Could use a trapezoidal integration for better accuracy.
        for i in 0..(self.int_wall_points.len() - 1) {
            let point0: &(f32, f32) = &self.int_wall_points[i+0];
            let point1: &(f32, f32) = &self.int_wall_points[i+1];

            let x_outer: f32 = (point0.0 + point1.0) / 2.0;
            let x_inner: f32 = (x_outer - self.wall_thickness.value_true).max(0.0);
            let y_avg: f32 = (point1.1 + point0.1) / 2.0;
            let height: f32 = point1.1 - point0.1;

            let patch_volume: f32 = f32::consts::PI * (x_outer.powi(2) - x_inner.powi(2)) * height;
            let patch_mass: f32 = patch_volume * self.material.density.value_true;
            let patch_ixx: f32 = 0.5 * patch_mass * (x_outer.powi(2) + x_inner.powi(2));
            let patch_iyy: f32 = patch_mass * (((x_outer.powi(2) + x_inner.powi(2)) / 4.0) + (height * height / 12.0));

            material_volume += patch_volume;
            mass += patch_mass;
            ixx += patch_ixx;
            iyy += patch_iyy + patch_mass * y_avg * y_avg;
            cg += patch_mass * y_avg;
            
            planform += x_outer * height * 2.0;
            planform_centroid += (x_outer * height * 2.0) * y_avg;
            aero_volume += f32::consts::PI * x_outer.powi(2) * height;
        }
        cg /= mass;
        iyy -= mass * cg * cg;
        planform_centroid /= planform;

        // Apply computed values to section unitvalues.
        self.int_mass_properties.wall_volume.value_true = material_volume; self.int_mass_properties.wall_volume.update_ui();
        self.int_mass_properties.mass.value_true = mass; self.int_mass_properties.mass.update_ui();
        self.int_mass_properties.i_rotational.value_true = ixx; self.int_mass_properties.i_rotational.update_ui();
        self.int_mass_properties.i_longitudinal.value_true = iyy; self.int_mass_properties.i_longitudinal.update_ui();
        self.int_mass_properties.cg.value_true = cg; self.int_mass_properties.cg.update_ui();

        // Get relevant areas for aero calcs.
        self.int_area_aft.value_true = 0.25 * f32::consts::PI * self.diameter_aft.value_true.powi(2); self.int_area_aft.update_ui();
        self.int_area_fore.value_true = 0.25 * f32::consts::PI * self.diameter_fore.value_true.powi(2); self.int_area_fore.update_ui();
        self.int_area_planform.value_true = planform; self.int_area_planform.update_ui();
        self.int_area_planform_centroid.value_true = planform_centroid; self.int_area_planform_centroid.update_ui();
        self.int_aero_volume.value_true = aero_volume; self.int_aero_volume.update_ui();

        // TODO Update components.
    }
    // Default values with intialization for internals.
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
            int_mass_properties: MassProperties::default(),
            int_area_aft:  UnitValue::m2(0.0),
            int_area_fore: UnitValue::m2(0.0),
            int_area_planform: UnitValue::m2(0.0),
            int_area_planform_centroid: UnitValue::meters(0.0),
            int_aero_volume: UnitValue::m3(0.0)
        };
        section.update_internal();
        return section;
    }
    
    /* -------------------------------------------------------------------------- */
    /*                                Aerodynamics                                */
    /* -------------------------------------------------------------------------- */
    // Aero coefficients.
    // (See OpenRocket technical docs)
    pub fn cna(&self, alpha: f32, mach: f32, aref: f32, lref: f32) -> f32 {   
        // TODO Check inverted behavior.

        let k: f32 = 1.1;
        let correction: f32 = if alpha == 0.0 { 1.0 } else { alpha.sin() / alpha };

        let cna_base: f32 = (2.0 / aref) * (self.int_area_aft.value_true - self.int_area_fore.value_true) * correction;
        let cna_body_lift: f32 = k * (self.int_area_planform.value_true / aref) * correction * alpha.sin();

        let mut cna_components: f32 = 0.0;
        for generic_component in &self.components {
            match generic_component {
                RocketComponent::FinSet(component) => { cna_components += component.cna(alpha, mach, aref, lref, self.diameter_aft.value_true / 2.0); },
                _ => {}
            }
        }
        return cna_base + cna_body_lift + cna_components;
    }
    pub fn cma(&self, alpha: f32, _mach: f32, aref: f32, lref: f32) -> f32 {
        // TODO Check inverted behavior.

        let k: f32 = 1.1;
        let correction: f32 = if alpha == 0.0 { 1.0 } else { alpha.sin() / alpha };

        let cma_base: f32 = correction * (2.0 / (aref * lref)) * (self.length.value_true * self.int_area_aft.value_true - self.int_aero_volume.value_true);
        let cma_body_lift: f32 = 0.0; // TODO ??? whats going on with body lift and cp : self.int_area_planform_centroid.value_true * k * (self.int_area_planform.value_true / aref) * correction * alpha.sin();
        return cma_base + cma_body_lift;
    }
    pub fn cp(&self, alpha: f32, mach: f32, aref: f32, lref: f32) -> f32 { 
        // TODO Check inverted behavior.
        // TODO CP estimate differs from openrocket as angle increases?

        let cna: f32 = self.cna(alpha, mach, aref, lref);
        let cma: f32 = self.cma(alpha, mach, aref, lref);
        if (self.diameter_aft.value_true - self.diameter_fore.value_true).abs() < 0.000001 { return 0.0; }
        else { return lref * cma / cna; }
    }

    /* -------------------------------------------------------------------------- */
    /*                                  Graphics                                  */
    /* -------------------------------------------------------------------------- */
    // Returns vector of lines made up of Vec2's that represent this section's lines (including components)
    pub fn draw(&self) -> Vec<Vec<Vec2>> {
        // Section lines.
        let mut paths: Vec<Vec<Vec2>> = vec![];
        paths.push(vec![
            vec2(self.diameter_fore.value_true / 2.0, 0.0), 
            vec2(-self.diameter_fore.value_true / 2.0, 0.0)
        ]);
        paths.push(vec![
            vec2(self.diameter_aft.value_true / 2.0, self.length.value_true), 
            vec2(-self.diameter_aft.value_true / 2.0, self.length.value_true)
        ]);
        let path_right: Vec<Vec2> = self.int_wall_points.clone().iter().map(|f| { vec2(f.0, f.1) }).collect();
        let path_left: Vec<Vec2> = self.int_wall_points.clone().iter().map(|f| { vec2(f.0 * -1.0, f.1) }).collect();
        paths.push(path_right);
        paths.push(path_left);

        // Component lines.
        for generic_component in &self.components {
            paths.append(&mut generic_component.draw(self));
        }
        return paths;
    }
}

/* -------------------------------------------------------------------------- */
/*                              Generic Component                             */
/* -------------------------------------------------------------------------- */
// Interface type for adressing rocket components as a generic type.
#[derive(Byteable, Debug, EnumAsInner)]
#[byteable(io_only)]
pub enum RocketComponent {
    FinSet(RocketComponentFinSet),
    GenericMass(RocketComponentGenericMass),
    Parachute(RocketComponentParachute),
}
impl RocketComponent {
    pub fn draw(&self, parent: &RocketSection) -> Vec<Vec<Vec2>> {
        match self {
            RocketComponent::FinSet(c) => c.draw(parent),
            RocketComponent::GenericMass(c) => c.draw(parent),
            RocketComponent::Parachute(c) => c.draw(parent),
        }
    }
}