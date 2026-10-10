/* -------------------------------------------------------------------------- */
/*                                   Curves                                   */
/* -------------------------------------------------------------------------- */
// Generic code for sampling curves.
// For a given profile, if there is an initial bool it represents whether the curve is flipped along its length.
// The f32 if present will represent some shape parameter for the curve.

use core::f32;
use byteable::Byteable;

#[derive(Byteable, PartialEq)]
#[byteable(io_only)]
pub enum CurveProfile {
    Conical,
    Ogive(bool, f32),
    Elliptical(bool),
    Parabolic(bool, f32),
    Haack(bool, f32)
}
impl CurveProfile {
    // Logic for flipping radii and x values for changing tangency.
    pub fn flip_coordinates(flipped: bool, r0: f32, r1: f32, l: f32, x: f32) -> (f32, f32, f32, f32) {        
        if !flipped { return (r0, r1, l, x); }

        let x: f32 = l - x;
        let temp: f32 = r1;
        let r1: f32 = r0;
        let r0: f32 = temp;
        return (r0, r1, l, x);
    }
    // Specific curve sample functions.
    pub fn sample_conical(r0: f32, r1: f32, l: f32, x: f32) -> f32 {
        // Straight line with a slope.
        let r: f32 = r1 - r0;
        return (x * r / l) + r0;
    }
    pub fn sample_ogive(flipped: bool, k: f32, r0: f32, r1: f32, l: f32, x: f32) -> f32 {
        // If R1=R0 curve is straight so we use conical logic to avoid singularities.
        if r1 == r0 { return Self::sample_conical(r0, r1, l, x); }
        // Same for K=0.
        if k == 0.0 { return Self::sample_conical(r0, r1, l, x); }

        // Handle curve flipping.
        let (r0, r1, l, x) = Self::flip_coordinates(flipped, r0, r1, l, x);

        // Secant curve.
        let direction: f32 = if r1 - r0 >= 0.0 { 1.0 } else { -1.0 };
        let r: f32 = (r1 - r0).abs();
        let rho_tangent: f32 = (r*r + l*l) / (2.0*r);
        let rho: f32 = rho_tangent / k;
        let alpha: f32 = ((r*r + l*l).sqrt() / (2.0*rho)).acos() - (r/l).atan();
        return ((rho*rho - (rho*alpha.cos() - x).powi(2)).sqrt() - rho*alpha.sin()) * direction + r0;
    }
    pub fn sample_elliptical(flipped: bool, r0: f32, r1: f32, l: f32, x: f32) -> f32 {
        // If R1=R0 curve is straight so we use conical logic to avoid singularities.
        if r1 == r0 { return Self::sample_conical(r0, r1, l, x); }

        // Handle curve flipping.
        let (r0, r1, l, x) = Self::flip_coordinates(flipped, r0, r1, l, x);

        // Elliptical curve.
        let r: f32 = r1 - r0;
        return ((r * (2.0*l*x - x*x).sqrt()) / l) + r0;
    }
    pub fn sample_parabolic(flipped: bool, k: f32, r0: f32, r1: f32, l: f32, x: f32) -> f32 {  
        // If R1=R0 curve is straight so we use conical logic to avoid singularities.
        if r1 == r0 { return Self::sample_conical(r0, r1, l, x); }

        // Handle curve flipping.
        let (r0, r1, l, x) = Self::flip_coordinates(flipped, r0, r1, l, x);

        // Parabolic curve.
        let r: f32 = r1 - r0;
        return r*(((2.0*x/l) - k*(x/l)*(x/l)) / (2.0-k)) + r0;
    }
    pub fn sample_haack(flipped: bool, k: f32, r0: f32, r1: f32, l: f32, x: f32) -> f32 {
        // If R1=R0 curve is straight so we use conical logic to avoid singularities.
        if r1 == r0 { return Self::sample_conical(r0, r1, l, x); }

        // Handle curve flipping.
        let (r0, r1, l, x) = Self::flip_coordinates(flipped, r0, r1, l, x);

        // Haack curve.
        let r: f32 = r1 - r0;
        let t: f32 = (1.0 - 2.0*x/l).acos();
        return (r / f32::consts::PI.sqrt()) * (t - (2.0*t).sin()/2.0 + k*(t.sin()).powi(3)).sqrt() + r0;
    }
    // Generic curve sample function.
    pub fn sample(&self, r0: f32, r1: f32, l: f32, x: f32) -> f32{
        match *self {
            CurveProfile::Conical => Self::sample_conical(r0, r1, l, x),
            CurveProfile::Ogive(flipped, k) => Self::sample_ogive(flipped, k, r0, r1, l, x),
            CurveProfile::Elliptical(flipped) => Self::sample_elliptical(flipped, r0, r1, l, x),
            CurveProfile::Parabolic(flipped, k) => Self::sample_parabolic(flipped, k, r0, r1, l, x),
            CurveProfile::Haack(flipped, k) => Self::sample_haack(flipped, k, r0, r1, l, x)
        }
    }
}