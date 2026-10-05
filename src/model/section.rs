use core::f32;

use enum_as_inner::EnumAsInner;
use crate::units::*;
use crate::materials::*;
use crate::model::finset::*;

/* -------------------------------------------------------------------------- */
/*                                  Sections                                  */
/* -------------------------------------------------------------------------- */
// The tubes, cones, and transitions that make up the body.
// These can all be described as an aft and fore diameter, constant thickness wall, and a length.
// Body tubes have the same aft and fore thickness, and nose cones have no fore diameter.
// They also have a "curve" variable which describes the shape of the curve which transitions between sections.
// They each have components that can be things like parachutes and fins.

/* ------------------------------- Definitions ------------------------------ */
#[derive(PartialEq)]
pub enum RocketSectionCurveSpec {
    // TODO More curves pending.
    Conical,
    OgiveAft,
    OgiveFore
}
pub struct RocketSection { 
    pub length: UnitValue, 
    pub diameter_fore: UnitValue, 
    pub diameter_aft: UnitValue, 
    pub wall_thickness: UnitValue,
    pub curve: RocketSectionCurveSpec,
    pub tapered: bool,

    pub copy_diameter_fore: bool,
    pub copy_diameter_aft: bool,

    pub custom_material: bool,
    pub material: MaterialSpec,

    pub components: Vec<RocketComponent>
}

/* ----------------------------- Implementation ----------------------------- */
impl RocketSection {
    // Top-view cross sectional area in square meters of a given spot in the cross section.
    // fn area_aft(&self) -> f32 { 0.25 * f32::consts::PI * self.diameter_aft.value_true.powi(2) }
    // fn area_fore(&self) -> f32 { 0.25 * f32::consts::PI * self.diameter_fore.value_true.powi(2) }
    // // Side-view cross sectional area in square meters.
    // fn area_planform(&self) -> f32 { 0.0 } // TODO
    // // Volume in cubic meters.
    fn volume(&self) -> f32 { 0.0 } // TODO
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
    pub fn default() -> RocketSection { RocketSection { 
        length: UnitValue::inches(30.0), 
        diameter_fore: UnitValue::inches(3.0), diameter_aft: UnitValue::inches(3.0), 
        wall_thickness: UnitValue::inches(0.08), 
        curve: RocketSectionCurveSpec::Conical, 
        tapered: false, 
        copy_diameter_fore: false, 
        copy_diameter_aft: false,
        components: vec![],
        custom_material: false,
        material: MaterialSpec::default()
    }}
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