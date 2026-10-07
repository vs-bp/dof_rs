use strum_macros::EnumIter;

/* ------------------------------ Unit Handling ----------------------------- */
// Many display and modelling variables will have configurable units.
// For each type of unit, the basic idea is to have a "true" value, a "ui" value, and an enum
// which defines the unit the ui value is in while the true value is always in a fixed (SI) unit.
//
// UnitTypes define the actual unit, kind defines whether its a unit of length, volume, ect for drawing the selector.
// UnitValues have a type and kind, where the kind determines what type values can be used (done in code) and the UnitValue
// type itself handles updates to either true or ui unit values and conversions between types. (visible to the user through the selector)

#[derive(Debug, Copy, Clone, PartialEq, EnumIter)]
pub enum UnitKind {
    Length,
    Area,
    Volume,
    Density,
    Mass,
    Inertia,
    Pressure
}

#[derive(Debug, Copy, Clone, PartialEq, EnumIter)]
pub enum UnitType {
    Foot, Inch, Millimeter, Centimeter, Meter,
    Pascal, Kilopascal, Megapascal, Gigapascal, psi, ksi,
    KgPerCubicMeter, OzPerCubicInch,
    Kg, G, Oz, Lb,
    CubicMeter,
    KgSquareMeter,
    SquareMeter
}
impl UnitType {
    // Return suffix for current enum type as a string.
    pub fn suffix(&self) -> String {
        match self {
            UnitType::Foot => { "ft".to_owned() },
            UnitType::Inch => { "in".to_owned() },
            UnitType::Millimeter => { "mm".to_owned() },
            UnitType::Centimeter => { "cm".to_owned() },
            UnitType::Meter => { "m".to_owned() },
            UnitType::Gigapascal => { "GPa".to_owned() },
            UnitType::Megapascal => { "MPa".to_owned() },
            UnitType::Kilopascal => { "KPa".to_owned() },
            UnitType::Pascal => { "Pa".to_owned() },
            UnitType::psi => { "psi".to_owned() },
            UnitType::ksi => { "ksi".to_owned() },
            UnitType::KgPerCubicMeter => { "kg/(m^3)".to_owned() },
            UnitType::OzPerCubicInch => { "oz/(in^3)".to_owned() },
            UnitType::Kg => { "kg".to_owned() },
            UnitType::G => { "g".to_owned() },
            UnitType::Oz => { "oz".to_owned() },
            UnitType::Lb => { "lb".to_owned() },
            UnitType::CubicMeter => { "m^3".to_owned() },
            UnitType::KgSquareMeter => { "kg * m^2".to_owned() },
            UnitType::SquareMeter => { "m^2".to_owned() }
        }
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct UnitValue {
    pub value_true: f32,    // meters.
    pub value_ui: f32,      // Any unit.
    pub unit: UnitType
}
impl UnitValue {
    // Constructor with default unit and dimension.
    pub fn meters(value_meters: f32) -> UnitValue { UnitValue { value_true: value_meters, value_ui: value_meters, unit: UnitType::Meter }}
    pub fn inches(value_inches: f32) -> UnitValue { UnitValue { value_true: value_inches * 0.0254, value_ui: value_inches, unit : UnitType::Inch }}
    pub fn kg_m3(value_kg_m3: f32) -> UnitValue { UnitValue { value_true: value_kg_m3, value_ui: value_kg_m3, unit : UnitType::KgPerCubicMeter }}
    pub fn pascals(value_pascals: f32) -> UnitValue { UnitValue { value_true: value_pascals, value_ui: value_pascals, unit : UnitType::Pascal }}
    pub fn m3(value_m3: f32) -> UnitValue { UnitValue { value_true: value_m3, value_ui: value_m3, unit : UnitType::CubicMeter }}
    pub fn kg(value_kg: f32) -> UnitValue { UnitValue { value_true: value_kg, value_ui: value_kg, unit : UnitType::Kg }}
    pub fn kgm2(value_kgm2: f32) -> UnitValue { UnitValue { value_true: value_kgm2, value_ui: value_kgm2, unit : UnitType::KgSquareMeter }}
    pub fn m2(value_kgm2: f32) -> UnitValue { UnitValue { value_true: value_kgm2, value_ui: value_kgm2, unit : UnitType::SquareMeter }}
    // Sets value_ui on unit change to conserve value_true.
    pub fn update_ui(&mut self) {
        self.value_ui = match self.unit {
            UnitType::Meter => self.value_true,
            UnitType::Millimeter => self.value_true * 1000.0,
            UnitType::Centimeter => self.value_true * 100.0,
            UnitType::Inch => self.value_true / 0.0254,
            UnitType::Foot => (self.value_true / 0.0254) / 12.0,
            UnitType::Gigapascal => self.value_true / 1000000000.0,
            UnitType::Megapascal => self.value_true / 1000000.0,
            UnitType::Kilopascal => self.value_true / 1000.0,
            UnitType::Pascal => self.value_true,
            UnitType::psi => self.value_true * 0.0001450377,
            UnitType::ksi => self.value_true * 0.0000001450377,
            UnitType::KgPerCubicMeter => self.value_true,
            UnitType::OzPerCubicInch => self.value_true * 0.00057803668683244,
            UnitType::Kg => self.value_true,
            UnitType::G => self.value_true * 1000.0,
            UnitType::Oz => self.value_true * 35.27396,
            UnitType::Lb => self.value_true * 2.204623,
            UnitType::CubicMeter => self.value_true,
            UnitType::KgSquareMeter => self.value_true,
            UnitType::SquareMeter => self.value_true
        };
    }
    // Sets value_true to what would be expected from value_ui and unit.
    pub fn update_true(&mut self) { 
        self.value_true = match self.unit {
            UnitType::Meter => self.value_ui,
            UnitType::Millimeter => self.value_ui / 1000.0,
            UnitType::Centimeter => self.value_ui / 100.0,
            UnitType::Inch => self.value_ui * 0.0254,
            UnitType::Foot => self.value_ui * 12.0 * 0.0254,
            UnitType::Gigapascal => self.value_ui * 1000000000.0,
            UnitType::Megapascal => self.value_ui * 1000000.0,
            UnitType::Kilopascal => self.value_ui * 1000.0,
            UnitType::Pascal => self.value_ui * 1.0,
            UnitType::psi => self.value_ui * 0.0001450377,
            UnitType::ksi => self.value_ui * 0.0000001450377,
            UnitType::KgPerCubicMeter => self.value_ui,
            UnitType::OzPerCubicInch => self.value_ui / 0.00057803668683244,
            UnitType::Kg => self.value_ui,
            UnitType::G => self.value_ui / 1000.0,
            UnitType::Oz => self.value_ui / 35.27396,
            UnitType::Lb => self.value_ui / 2.204623,
            UnitType::CubicMeter => self.value_ui,
            UnitType::KgSquareMeter => self.value_ui,
            UnitType::SquareMeter => self.value_ui
        };
    }
}