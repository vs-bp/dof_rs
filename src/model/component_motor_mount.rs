use byteable::Byteable;
use egui::{Color32, ComboBox, ScrollArea, Ui, Vec2, vec2};
use elegance::{CollapsingSection, MetricSlider, RangeSlider, TextInput};

use crate::{App, model::{int_motors::Motor, int_section::RocketSection, int_ui_base::{UnitRangeSpec, ViewerLine, show_unit_value_field}, int_units::{UnitKind, UnitValue}}};

#[derive(Byteable, Debug)]
#[byteable(io_only)]
pub struct RocketComponentMotorMount {
    pub position_offset: UnitValue,
    pub motor: Motor,
    // Impulse filter selection.
    // 1 = A, 15 = O, 16 = P+
    pub impulse_filter_low: f32,
    pub impulse_filter_high: f32,
    pub name_filter: String
}
impl RocketComponentMotorMount {
    /* -------------------------------------------------------------------------- */
    /*                               Internal Values                              */
    /* -------------------------------------------------------------------------- */
    pub fn default() -> RocketComponentMotorMount { RocketComponentMotorMount { 
        position_offset: UnitValue::inches(0.0), 
        motor: Motor::default(),
        // Impulse filter selection.
        // 1 = A, 15 = O, 16 = P+
        impulse_filter_low: 1.0,
        impulse_filter_high: 16.0,
        name_filter: "".to_owned()
    }}

    /* -------------------------------------------------------------------------- */
    /*                                     UI                                     */
    /* -------------------------------------------------------------------------- */
    // Shows editable fields and mass properties. (Called on selection in model tree)
    pub fn show(ui: &mut Ui, section_idx: usize, component_idx: usize, app: &mut App) {
        // Offset slider.
        let component: &mut RocketComponentMotorMount = &mut app.model.sections[section_idx].components[component_idx].as_motor_mount_mut().unwrap();
        show_unit_value_field(ui, section_idx*1000 + component_idx, UnitKind::Length, "Offset".to_owned(),  &mut component.position_offset, UnitRangeSpec::FinOffset, None, "");

        // Selector combo.
        CollapsingSection::new("motorselectordropdown".to_owned() + &(section_idx * 1000 + component_idx).to_string(), "Motor Selector").show(ui, |ui| {
            // Impulse filter selection.
            // 1 = A, 15 = O, 16 = P+
            let class_low: &str = match component.impulse_filter_low as usize {
                1 => "A", 2 => "B", 3 => "C", 4 => "D", 5 => "E",
                6 => "F", 7 => "G", 8 => "H", 9 => "I", 10 => "J",
                11 => "K", 12 => "L", 13 => "M", 14 => "N", 15 => "O",
                16 => "P+", _ => "ERR"
            };
            let class_high: &str = match component.impulse_filter_high as usize {
                1 => "A", 2 => "B", 3 => "C", 4 => "D", 5 => "E",
                6 => "F", 7 => "G", 8 => "H", 9 => "I", 10 => "J",
                11 => "K", 12 => "L", 13 => "M", 14 => "N", 15 => "O",
                16 => "P+", _ => "ERR"
            };
            ui.add(
                RangeSlider::new(&mut component.impulse_filter_low, &mut component.impulse_filter_high, 1.0..=15.0)
                .step(1.0)
                .decimals(0)
                .label("Impulse Range: ".to_owned() + class_low + " - " + class_high)
                .ticks(15)
                .show_tick_labels(false)
                .show_value(false)
            );

            // Name filter input.
            ui.horizontal(|ui| { 
                ui.label("Name: ");
                ui.add(TextInput::new(&mut component.name_filter));
            });
            
            // Show filtered results.
            ScrollArea::new([false, true]).max_height(300.0).show(ui, |ui| {
                for motor in &app.motors {
                    let impulse_class_letter: char = motor.name.chars().next().unwrap();
                    let impulse_class_number: usize = match impulse_class_letter {
                        'A' => 1, 'B' => 2, 'C' => 3, 'D' => 4, 'E' => 5,
                        'F' => 6, 'G' => 7, 'H' => 8, 'I' => 9, 'J' => 10,
                        'K' => 11, 'L' => 12, 'M' => 13, 'N' => 14, 'O' => 15,
                        _ => 16
                    };
                    if impulse_class_number < component.impulse_filter_low as usize { continue; }
                    if impulse_class_number > component.impulse_filter_high as usize { continue; }
                    if !(motor.name.contains(&component.name_filter)) { continue; }
                    let checked: bool = motor.name == component.motor.name;
                    if ui.selectable_label(checked, motor.name.clone()).clicked() { component.motor = motor.clone(); }
                }
            });
        });

        // Plot.
        CollapsingSection::new("motorthrustcurvedropdown".to_owned() + &(section_idx * 1000 + component_idx).to_string(), "Motor Thrust Curve").show(ui, |ui| {
            egui_plot::Plot::new("motorthrustcurve".to_owned() + &(section_idx * 1000 + component_idx).to_string())
            .legend(egui_plot::Legend::default())
            .show_axes(true)
            .show_grid(true)
            .height(300.0)
            .show(ui, |plot_ui| {
                plot_ui.line(egui_plot::Line::new("Thrust (N) vs. Time (s)", egui_plot::PlotPoints::from_iter(component.motor.thrust_curve.iter().map(|d| [(*d).0 as f64, (*d).1 as f64]))));
            });
        });
    }

    /* -------------------------------------------------------------------------- */
    /*                                  Graphics                                  */
    /* -------------------------------------------------------------------------- */
    // Returns vector of lines made up of Vec2's that represent this component's lines.
    pub fn draw(&self, parent: &RocketSection) -> Vec<ViewerLine> {
        // TODO Variable color?

        // Basic coordinates.
        let offset: f32 = parent.length.value_true;
        let c1: Vec2 = vec2(-self.motor.diameter_mm / 2000.0, -self.position_offset.value_true + offset);
        let c2: Vec2 = vec2(self.motor.diameter_mm / 2000.0, -self.position_offset.value_true + offset);
        let c3: Vec2 = vec2(self.motor.diameter_mm / 2000.0, -self.position_offset.value_true + offset - self.motor.length_mm / 1000.0);
        let c4: Vec2 = vec2(-self.motor.diameter_mm / 2000.0, -self.position_offset.value_true + offset - self.motor.length_mm / 1000.0);

        // Output.
        const MOTOR_LINE_ORDER: usize = 11000;
        return vec![ViewerLine::Solid(Color32::PURPLE, MOTOR_LINE_ORDER, vec![c1,c2,c3,c4,c1])];
    }
}