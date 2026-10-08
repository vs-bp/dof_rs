use std::{collections::HashMap, ops::{RangeInclusive}};
use elegance::{ Checkbox, Slider, TextInput, egui::{ ComboBox, DragValue, Label, Ui, CollapsingHeader } };
use crate::units::*;
use crate::materials::*;
use crate::model::mass::*;
use crate::model::curves::*;

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
    ModulusShear,
    WallThickness
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
            UnitRangeSpec::WallThickness => match unit {
                UnitType::Inch => 0.0..=0.50,
                UnitType::Foot => 0.0..=0.05,
                UnitType::Meter => 0.0..=0.013,
                UnitType::Centimeter => 0.0..=1.3,
                UnitType::Millimeter => 0.0..=13.0,
                _ => 0.0..=0.0
            }
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
            },
            UnitKind::Mass => {
                ui.selectable_value(value, UnitType::Kg, UnitType::Kg.suffix());
                ui.selectable_value(value, UnitType::G, UnitType::G.suffix());
                ui.selectable_value(value, UnitType::Oz, UnitType::Oz.suffix());
                ui.selectable_value(value, UnitType::Lb, UnitType::Lb.suffix());
            },
            UnitKind::Volume => {
                ui.selectable_value(value, UnitType::CubicMeter, UnitType::CubicMeter.suffix());
            },
            UnitKind::Inertia => {
                ui.selectable_value(value, UnitType::KgSquareMeter, UnitType::KgSquareMeter.suffix());
            },
            UnitKind::Area => {
                ui.selectable_value(value, UnitType::SquareMeter, UnitType::SquareMeter.suffix());
            }
        }
    });
    return previous != *value;
}

/* ----------------------------- Value Displays ----------------------------- */
// Duo of numeric value and slider within given range.
pub fn show_range_value_field(ui: &mut Ui, value: &mut f32, range: RangeInclusive<f32>, label: String, suffix: String) {
    let width: f32 = 300.0;
    ui.horizontal(|ui| {
        ui.add(DragValue::new(value).suffix(suffix.clone()).fixed_decimals(3));
        ui.separator();
        ui.add(Slider::new(value, range).suffix(suffix.clone()).decimals(3).desired_width(width).label(label));
    });
}
// Trio of numeric value, slider, checkbox, and unit selector.
// Refreshes true unit value per frame and auto adjusts ui value on unit change.
pub fn show_unit_value_field(
    ui: &mut Ui, id_source: usize, 
    unit_kind: UnitKind, label: String, value: &mut UnitValue, range_spec: UnitRangeSpec,
    checkbox: Option<&mut bool>, checkbox_label: &str
) {
    let width: f32 = 300.0;
    let mut suffix: String = value.unit.suffix();
    let mut range: RangeInclusive<f32> = range_spec.resolve(value.unit);

    ui.horizontal(|ui| {
        // Unit change calls update_ui.
        if show_unit_selector(ui, id_source, unit_kind, &label, &mut value.unit) { 
            value.update_ui(); 
            suffix = value.unit.suffix();
            range = range_spec.resolve(value.unit);
        }

        ui.separator();
        ui.add(DragValue::new(&mut value.value_ui).suffix(suffix.clone()).fixed_decimals(3));
        ui.separator(); 
        ui.add(Slider::new(&mut value.value_ui, range).suffix(suffix.clone()).decimals(3).desired_width(width).label(label));
        if checkbox.is_some() { 
            ui.separator(); 
            ui.add(Checkbox::new(checkbox.unwrap(), checkbox_label));
        }
        
        // Each frame calls update_true but only after update_ui had its chance to run.
        value.update_true();
    });
}
// Duo of numeric value label and unit selector.
// Auto adjusts ui value on unit change.
// TODO Checkbox to include subcomponents later?
pub fn show_unit_value_label(
    ui: &mut Ui, id_source: usize, 
    unit_kind: UnitKind, label: String, value: &mut UnitValue
) {
    let mut suffix: String = value.unit.suffix();

    ui.columns(3, |columns| {
        // Name label.
        columns[0].add(Label::new(&label));
        // Value label.
        columns[1].add(Label::new(value.value_ui.to_string() + " " + &suffix));
        // Unit change calls update_ui.
        if show_unit_selector(&mut columns[2], id_source, unit_kind, &label, &mut value.unit) { 
            value.update_ui(); 
            suffix = value.unit.suffix();
        }
    });
}

/* -------------------------------- Materials ------------------------------- */
// Dropdown for selecting material and boxes for defining a custom one for that part.
pub fn show_material_selector(ui: &mut Ui, id_source: usize, material_value: &mut Material, custom_material_flag: &mut bool, materials: &HashMap<String, Material>) {
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
        show_unit_value_field(ui, id_source, UnitKind::Density, "Density".to_string(), &mut material_value.density, UnitRangeSpec::MaterialDensity, None, "");
        show_unit_value_field(ui, id_source, UnitKind::Pressure, "Young's Modulus".to_string(), &mut material_value.modulus_youngs, UnitRangeSpec::ModulusYoungs, None, "");
        show_unit_value_field(ui, id_source, UnitKind::Pressure, "Shear Modulus".to_string(), &mut material_value.modulus_shear, UnitRangeSpec::ModulusShear, None, "");
    }
}

/* ----------------------------- Mass Properties ---------------------------- */
pub fn show_mass_properties(ui: &mut Ui, id_source: usize, mass_properties: &mut MassProperties) {
    show_unit_value_label(ui, id_source, UnitKind::Volume, "Volume".to_owned(), &mut mass_properties.volume);
    show_unit_value_label(ui, id_source, UnitKind::Mass, "Mass".to_owned(), &mut mass_properties.mass);
    show_unit_value_label(ui, id_source, UnitKind::Length, "CG".to_owned(), &mut mass_properties.cg);
    show_unit_value_label(ui, id_source, UnitKind::Inertia, "Irot".to_owned(), &mut mass_properties.i_rotational);
    show_unit_value_label(ui, id_source, UnitKind::Inertia, "Ilong".to_owned(), &mut mass_properties.i_longitudinal);
}

/* --------------------------------- Curves --------------------------------- */
// Curve selector combo box.
pub fn show_curve_combo(ui: &mut Ui, curve_spec: &mut CurveProfile) {
    ui.horizontal(|ui| {
        ui.add(Label::new("Curve Profile"));
        ui.separator();
        let selected_text: &str = match *curve_spec {
            CurveProfile::Conical => "Conical",
            CurveProfile::Ogive(_,_) => "Ogive",
            CurveProfile::Elliptical(_) => "Elliptical",
            CurveProfile::Parabolic(_,_) => "Parabolic",
            CurveProfile::Haack(_,_) => "Haack"
        };
        ComboBox::new("curveselect", "").selected_text(selected_text).show_ui(ui, |ui| {
            ui.selectable_value(curve_spec, CurveProfile::Conical, "Conical");
            ui.selectable_value(curve_spec, CurveProfile::Ogive(false, 1.0), "Ogive");
            ui.selectable_value(curve_spec, CurveProfile::Elliptical(false), "Elliptical");
            ui.selectable_value(curve_spec, CurveProfile::Haack(false, 0.333), "Haack");
            ui.selectable_value(curve_spec, CurveProfile::Parabolic(false, 1.0), "Parabolic");
        });
        match curve_spec {
            CurveProfile::Conical => {},
            CurveProfile::Ogive(flipped, k) => {
                ui.add(Checkbox::new(flipped, "Flipped"));
                ui.separator();
                show_range_value_field(ui, k, 0.0..=1.0, "Shape Parameter".to_owned(), "".to_owned());
            },
            CurveProfile::Haack(flipped, k) => {
                ui.add(Checkbox::new(flipped, "Flipped"));
                ui.separator();
                show_range_value_field(ui, k, 0.0..=1.0, "Shape Parameter".to_owned(), "".to_owned());
            },
            CurveProfile::Parabolic(flipped, k) => {
                ui.add(Checkbox::new(flipped, "Flipped"));
                ui.separator();
                show_range_value_field(ui, k, 0.0..=1.0, "Shape Parameter".to_owned(), "".to_owned());
            },
            CurveProfile::Elliptical(flipped) => {
                ui.add(Checkbox::new(flipped, "Flipped"));
            }
        }
    });
}