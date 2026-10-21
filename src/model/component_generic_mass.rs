use byteable::Byteable;
use egui::Color32;
use egui::Ui;
use egui::Vec2;
use egui::vec2;
use elegance::CollapsingSection;
use elegance::ColorPicker;
use crate::App;
use crate::model::int_mass::MassProperties;
use crate::model::int_materials::Material;
use crate::model::int_section::*;
use crate::model::int_ui_base::*;
use crate::model::int_units::*;

/* -------------------------------------------------------------------------- */
/*                           Generic Mass Component                           */
/* -------------------------------------------------------------------------- */
// Generic mass component.

#[derive(Byteable, Debug)]
#[byteable(io_only)]
pub struct RocketComponentGenericMass {
    pub diameter_aft: UnitValue,
    pub diameter_fore: UnitValue,
    pub length: UnitValue,
    // TODO Override to solid with a checkbox.
    pub wall_thickness: UnitValue,
    pub material: Material,
    pub int_mass_properties: MassProperties,
    pub position_offset: UnitValue,
    pub viewer_color: Color32
}
impl RocketComponentGenericMass {
    /* -------------------------------------------------------------------------- */
    /*                               Internal Values                              */
    /* -------------------------------------------------------------------------- */
    pub fn default() -> RocketComponentGenericMass { RocketComponentGenericMass {
        diameter_aft: UnitValue::inches(1.5),
        diameter_fore: UnitValue::inches(1.5),
        length: UnitValue::inches(4.0),
        wall_thickness: UnitValue::inches(0.08),
        material: Material::default(),
        int_mass_properties: MassProperties::default(),
        position_offset: UnitValue::inches(0.0),
        viewer_color: Color32::LIGHT_RED
    }}
    pub fn update_internal(&mut self) {
        // TODO.
    }
    
    /* -------------------------------------------------------------------------- */
    /*                                     UI                                     */
    /* -------------------------------------------------------------------------- */
    // Shows editable fields and mass properties. (Called on selection in model tree)
    pub fn show(ui: &mut Ui, section_idx: usize, component_idx: usize, app: &mut App) {
        let component: &mut RocketComponentGenericMass = &mut app.model.sections[section_idx].components[component_idx].as_generic_mass_mut().unwrap();
        CollapsingSection::new("componentdimensions".to_owned() + &(section_idx*1000 + component_idx).to_string(), "Component Dimensions").show(ui, |ui| {
            show_unit_value_field(ui, section_idx*1000 + component_idx, UnitKind::Length, "Fore Diameter".to_owned(),  &mut component.diameter_fore, UnitRangeSpec::TubeDiameter, None, "");
            show_unit_value_field(ui, section_idx*1000 + component_idx, UnitKind::Length, "Aft Diameter".to_owned(),  &mut component.diameter_aft, UnitRangeSpec::TubeDiameter, None, "");
            show_unit_value_field(ui, section_idx*1000 + component_idx, UnitKind::Length, "Length".to_owned(),  &mut component.length, UnitRangeSpec::FinRoot, None, "");
            show_unit_value_field(ui, section_idx*1000 + component_idx, UnitKind::Length, "Offset".to_owned(),  &mut component.position_offset, UnitRangeSpec::FinOffset, None, "");
            show_unit_value_field(ui, section_idx*1000 + component_idx, UnitKind::Length, "Wall Thickness".to_owned(),  &mut component.wall_thickness, UnitRangeSpec::WallThickness, None, "");
            // TODO Move to UI base.
            ui.add(ColorPicker::new("componentcolor".to_owned() + &(section_idx*1000 + component_idx).to_string(), &mut component.viewer_color));
        });
        CollapsingSection::new("componentmassproperties".to_owned() + &(section_idx*1000 + component_idx).to_string(), "Component Properties").show(ui, |ui| {
            show_mass_properties(ui, section_idx*1000 + component_idx, &mut component.int_mass_properties);
        });
    }

    /* -------------------------------------------------------------------------- */
    /*                                  Graphics                                  */
    /* -------------------------------------------------------------------------- */
    // Returns vector of lines made up of Vec2's that represent this component's lines.
    pub fn draw(&self, parent: &RocketSection) -> Vec<ViewerLine> {        
        // Basic coordinates.

        // Outer walls.
        let offset: f32 = parent.length.value_true;
        let c1: Vec2 = vec2(-self.diameter_aft.value_true / 2.0, -self.position_offset.value_true + offset);
        let c2: Vec2 = vec2(self.diameter_aft.value_true / 2.0, -self.position_offset.value_true + offset);
        let c3: Vec2 = vec2(self.diameter_fore.value_true / 2.0, -self.position_offset.value_true + offset - self.length.value_true);
        let c4: Vec2 = vec2(-self.diameter_fore.value_true / 2.0, -self.position_offset.value_true + offset - self.length.value_true);

        // TODO Maybe inner walls?s

        // Output.
        const INTERNAL_COMPONENT_LINE_ORDER: usize = 9000;
        return vec![ViewerLine::Solid(self.viewer_color, INTERNAL_COMPONENT_LINE_ORDER, vec![c1,c2,c3,c4,c1])];
    }
}