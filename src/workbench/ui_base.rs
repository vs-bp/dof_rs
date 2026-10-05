use std::{collections::HashMap, ops::RangeInclusive};
use elegance::{ Checkbox, Slider, TextInput, egui::{ ComboBox, DragValue, Label, Ui } };
use strum::IntoEnumIterator;
use crate::units::*;
use crate::materials::*;
use crate::model::finset::*;
use crate::model::section::*;

/* --------------------------------- Ranges --------------------------------- */
// Range specification for draggable values (tube diam, tube length, motor diam, elastic/shear modulus, etc.)
pub enum UnitRangeSpec {
    TubeDiameter,
    TubeLength,
    FinRoot,
    FinTip,
    FinHeight,
    FinSweep,
    FinOffset,
    MaterialDensity,
    ModulusYoungs,
    ModulusShear
}
impl UnitRangeSpec {
    pub fn resolve(&self, unit: UnitType) -> RangeInclusive<f32> {
        match self {
            UnitRangeSpec::TubeDiameter => match unit {
                UnitType::Inch => 0.0..=12.0,
                UnitType::Foot => 0.0..=1.0,
                UnitType::Meter => 0.0..=0.3,
                UnitType::Centimeter => 0.0..=30.0,
                UnitType::Millimeter => 0.0..=300.0,
                _ => 0.0..=0.0
            },
            UnitRangeSpec::TubeLength => match unit {
                UnitType::Inch => 0.0..=72.0,
                UnitType::Foot => 0.0..=6.0,
                UnitType::Meter => 0.0..=2.0,
                UnitType::Centimeter => 0.0..=200.0,
                UnitType::Millimeter => 0.0..=2000.0,
                _ => 0.0..=0.0
            },
            UnitRangeSpec::FinRoot => match unit {
                UnitType::Inch => 0.0..=12.0,
                UnitType::Foot => 0.0..=1.0,
                UnitType::Meter => 0.0..=0.3,
                UnitType::Centimeter => 0.0..=30.0,
                UnitType::Millimeter => 0.0..=300.0,
                _ => 0.0..=0.0
            },
            UnitRangeSpec::FinTip => match unit {
                UnitType::Inch => 0.0..=6.0,
                UnitType::Foot => 0.0..=0.5,
                UnitType::Meter => 0.0..=0.15,
                UnitType::Centimeter => 0.0..=15.0,
                UnitType::Millimeter => 0.0..=150.0,
                _ => 0.0..=0.0
            },
            UnitRangeSpec::FinHeight => match unit {
                UnitType::Inch => 0.0..=12.0,
                UnitType::Foot => 0.0..=1.0,
                UnitType::Meter => 0.0..=0.3,
                UnitType::Centimeter => 0.0..=30.0,
                UnitType::Millimeter => 0.0..=300.0,
                _ => 0.0..=0.0
            },
            UnitRangeSpec::FinSweep => match unit {
                UnitType::Inch => 0.0..=12.0,
                UnitType::Foot => 0.0..=1.0,
                UnitType::Meter => 0.0..=0.3,
                UnitType::Centimeter => 0.0..=30.0,
                UnitType::Millimeter => 0.0..=300.0,
                _ => 0.0..=0.0
            },
            UnitRangeSpec::FinOffset => match unit {
                UnitType::Inch => -6.0..=6.0,
                UnitType::Foot => -0.5..=0.5,
                UnitType::Meter => -0.15..=0.15,
                UnitType::Centimeter => -15.0..=15.0,
                UnitType::Millimeter => -150.0..=150.0,
                _ => 0.0..=0.0
            },
            UnitRangeSpec::MaterialDensity => match unit {
                UnitType::KgPerCubicMeter => 0.0..=2000.0,
                UnitType::OzPerCubicInch => 0.0..=1.3,
                _ => 0.0..=0.0
            },
            UnitRangeSpec::ModulusYoungs => match unit {
                UnitType::Gigapascal => 0.0..=200.0,
                UnitType::Megapascal => 0.0..=200000.0,
                UnitType::Kilopascal => 0.0..=200000000.0,
                UnitType::Pascal => 0.0..=200000000000.0,
                UnitType::psi => 0.0..=30000000.0,
                UnitType::ksi => 0.0..=30000.0,
                _ => 0.0..=0.0
            },
            UnitRangeSpec::ModulusShear => match unit {
                UnitType::Gigapascal => 0.0..=200.0,
                UnitType::Megapascal => 0.0..=200000.0,
                UnitType::Kilopascal => 0.0..=200000000.0,
                UnitType::Pascal => 0.0..=200000000000.0,
                UnitType::psi => 0.0..=30000000.0,
                UnitType::ksi => 0.0..=30000.0,
                _ => 0.0..=0.0
            },
        }
    }
}

/* ------------------------------ Unit Selector ----------------------------- */
// Dropdown for selecting value unit.
// Returns true on change.
pub fn show_unit_selector(ui: &mut Ui, id_source: usize, unit_kind: UnitKind, field_label: &str, value: &mut UnitType) -> bool {
    let previous: UnitType = *value;
    ComboBox::new("unitselect".to_owned() + &id_source.to_string() + field_label, "").selected_text(value.suffix()).show_ui(ui, |ui| {
        match unit_kind {
            UnitKind::Length => {
                ui.selectable_value(value, UnitType::Foot, UnitType::Foot.suffix());
                ui.selectable_value(value, UnitType::Inch, UnitType::Inch.suffix());
                ui.selectable_value(value, UnitType::Meter, UnitType::Meter.suffix());
                ui.selectable_value(value, UnitType::Centimeter, UnitType::Centimeter.suffix());
                ui.selectable_value(value, UnitType::Millimeter, UnitType::Millimeter.suffix());
            },
            UnitKind::Pressure => {
                ui.selectable_value(value, UnitType::ksi, UnitType::ksi.suffix());
                ui.selectable_value(value, UnitType::psi, UnitType::psi.suffix());
                ui.selectable_value(value, UnitType::Gigapascal, UnitType::Gigapascal.suffix());
                ui.selectable_value(value, UnitType::Megapascal, UnitType::Megapascal.suffix());
                ui.selectable_value(value, UnitType::Kilopascal, UnitType::Kilopascal.suffix());
                ui.selectable_value(value, UnitType::Pascal, UnitType::Pascal.suffix());
            },
            UnitKind::Density => {
                ui.selectable_value(value, UnitType::KgPerCubicMeter, UnitType::KgPerCubicMeter.suffix());
                ui.selectable_value(value, UnitType::OzPerCubicInch, UnitType::OzPerCubicInch.suffix());
            }
        }
    });
    return previous != *value;
}

/* ----------------------------- Value Displays ----------------------------- */
// Trio of numeric value, slider, checkbox, and unit selector.
// Refreshes true value per frame.
pub fn show_unit_value(
    ui: &mut Ui, id_source: usize, 
    unit_kind: UnitKind, label: String, value: &mut UnitValue, range_spec: UnitRangeSpec,
    checkbox: Option<&mut bool>, checkbox_label: &str
) {
    let width: f32 = 300.0;
    let mut suffix: String = value.unit.suffix();
    let mut range: RangeInclusive<f32> = range_spec.resolve(value.unit);

    ui.horizontal(|ui| {
        if show_unit_selector(ui, id_source, unit_kind, &label, &mut value.unit) { 
            value.update_ui(); 
            suffix = value.unit.suffix();
            range = range_spec.resolve(value.unit);
        }
        ui.separator();
        if ui.add(DragValue::new(&mut value.value_ui).suffix(suffix.clone()).fixed_decimals(3)).changed() { value.update_true(); }
        ui.separator(); 
        if ui.add(Slider::new(&mut value.value_ui, range).suffix(suffix.clone()).decimals(3).desired_width(width).label(label)).changed() { value.update_true(); }
        if checkbox.is_some() { ui.separator(); ui.add(Checkbox::new(checkbox.unwrap(), checkbox_label)); }
    });
}

/* -------------------------------- Materials ------------------------------- */
// Dropdown for selecting material and boxes for defining a custom one for that part.
pub fn show_material_selector(ui: &mut Ui, id_source: usize, material_value: &mut MaterialSpec, custom_material_flag: &mut bool, materials: &HashMap<String, MaterialSpec>) {
    // Horizontal bar for material dropdown or custom material name.
    ui.horizontal(|ui| {
        ui.add(Checkbox::new(custom_material_flag, "Custom Material"));

        if *custom_material_flag {
            // Name if material is custom.
            ui.separator();
            ui.add(Label::new("Material Name "));
            ui.add(TextInput::new(&mut material_value.name).desired_width(200.0));
        } else {
            // Dropdown if material isnt custom.
            ui.separator();
            ui.add(Label::new("Material"));
            ComboBox::new("materialselect".to_owned() + &id_source.to_string(), "").selected_text(material_value.name.clone()).show_ui(ui, |ui| {
                for material in materials { ui.selectable_value(material_value, material.1.clone(), material.1.name.clone()); }
            });
        }
    });
    // Vertical section for settings if custom material set.
    if *custom_material_flag {
        show_unit_value(ui, id_source, UnitKind::Density, "Density".to_string(), &mut material_value.density, UnitRangeSpec::MaterialDensity, None, "");
        show_unit_value(ui, id_source, UnitKind::Pressure, "Young's Modulus".to_string(), &mut material_value.modulus_youngs, UnitRangeSpec::ModulusYoungs, None, "");
        show_unit_value(ui, id_source, UnitKind::Pressure, "Shear Modulus".to_string(), &mut material_value.modulus_shear, UnitRangeSpec::ModulusShear, None, "");
    }
}

/* --------------------------------- Curves --------------------------------- */
// Curve selector combo box.
pub fn show_curve_combo(ui: &mut Ui, curve_spec: &mut RocketSectionCurveSpec) {
    ui.horizontal(|ui| {
        ui.add(Label::new("Curve Profile"));
        ui.separator();
        let selected_text: &str = match *curve_spec {
            RocketSectionCurveSpec::Conical => "Conical",
            RocketSectionCurveSpec::OgiveFore => "Ogive (Fore)",
            RocketSectionCurveSpec::OgiveAft => "Ogive (Aft)"
        };
        ComboBox::new("curveselect", "").selected_text(selected_text).show_ui(ui, |ui| {
            ui.selectable_value(curve_spec, RocketSectionCurveSpec::Conical, "Conical");
            ui.selectable_value(curve_spec, RocketSectionCurveSpec::OgiveFore, "Ogive (Tangent to fore)");
            ui.selectable_value(curve_spec, RocketSectionCurveSpec::OgiveAft, "Ogive (Tangent to aft)");
        });
    });
}