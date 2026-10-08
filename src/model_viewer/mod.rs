use core::f32;
use elegance::egui::{ Vec2, vec2, Ui, Rect, Sense, CornerRadius, Stroke, Color32, StrokeKind, Pos2 };
use crate::model::RocketModel;
use crate::model::finset::*;
use crate::model::section;
use crate::model::section::*;
use crate::App;

/* -------------------------------------------------------------------------- */
/*                            Section Draw Function                           */
/* -------------------------------------------------------------------------- */
// Returns the vector of vec2 pairs which define the lines for the given section.
pub fn draw_section(draw_position: f32, section: &RocketSection) -> Vec<Vec<Vec2>> {
    let mut paths: Vec<Vec<Vec2>> = vec![];
    paths.push(vec![
        vec2(section.diameter_fore.value_true / 2.0, draw_position), 
        vec2(-section.diameter_fore.value_true / 2.0, draw_position)
    ]);
    paths.push(vec![
        vec2(section.diameter_aft.value_true / 2.0, draw_position + section.length.value_true), 
        vec2(-section.diameter_aft.value_true / 2.0, draw_position + section.length.value_true)
    ]);
    let path_right: Vec<Vec2> = section.int_wall_points.clone().iter().map(|f| { vec2(f.x, f.y + draw_position) }).collect();
    let path_left: Vec<Vec2> = section.int_wall_points.clone().iter().map(|f| { vec2(f.x * -1.0, f.y + draw_position) }).collect();
    paths.push(path_right);
    paths.push(path_left);
    return paths;
}


/* -------------------------------------------------------------------------- */
/*                          Component Draw Functions                          */
/* -------------------------------------------------------------------------- */
// Returns the vector of vec2 pairs which define the lines for the given component.
// Fins use draw_position but dont change it.
pub fn draw_component_fin(draw_position: f32, angle: f32, body_diameter: f32, component: &RocketComponentFinSet) -> Vec<Vec<Vec2>> {
    // TODO Only trapezoidal fins for now.
    // Basic coordinates.
    let mut c_root_low: Vec2 = vec2(-body_diameter / 2.0, draw_position - component.position_offset.value_true);
    let mut c_root_high: Vec2 = c_root_low + vec2(0.0, -component.chord_root.value_true);
    let mut c_tip_high: Vec2 = vec2(
        -component.height.value_true - body_diameter / 2.0, 
        draw_position - component.position_offset.value_true - component.chord_root.value_true + component.sweep.value_true
    );
    let mut c_tip_low: Vec2 = c_tip_high + vec2(0.0, component.chord_tip.value_true);

    // Transform y axes by angle.
    c_root_low.x *= angle.cos();
    c_root_high.x *= angle.cos();
    c_tip_low.x *= angle.cos();
    c_tip_high.x *= angle.cos();

    // Output.
    return vec![vec![c_root_low, c_tip_low, c_tip_high, c_root_high]];
}
pub fn draw_component_fin_set(draw_position: f32, component: &RocketComponentFinSet, parent: &RocketSection) -> Vec<Vec<Vec2>> {
    let mut paths: Vec<Vec<Vec2>> = vec![];
    for i in 0..component.count {
        paths.append(&mut draw_component_fin(draw_position, i as f32 * 2.0 * f32::consts::PI / component.count as f32, parent.diameter_aft.value_true, component));
    }
    return paths;
}
pub fn draw_component_parachute(draw_position: f32, component: &RocketComponentParachute, parent: &RocketSection) -> Vec<Vec<Vec2>> {
    // TODO Variable color?

    // Basic coordinates.
    let c1: Vec2 = vec2(-component.packed_diameter.value_true / 2.0, -component.position_offset.value_true + draw_position);
    let c2: Vec2 = vec2(component.packed_diameter.value_true / 2.0, -component.position_offset.value_true + draw_position);
    let c3: Vec2 = vec2(component.packed_diameter.value_true / 2.0, -component.position_offset.value_true + draw_position - component.packed_length.value_true);
    let c4: Vec2 = vec2(-component.packed_diameter.value_true / 2.0, -component.position_offset.value_true + draw_position - component.packed_length.value_true);

    // Output.
    return vec![vec![c1,c2,c3,c4,c1]];
}
// Generic function to draw any given component based off its type.
pub fn draw_component(draw_position: f32, generic_component: &RocketComponent, parent: &RocketSection) -> Vec<Vec<Vec2>> {
    match generic_component {
        RocketComponent::FinSet(component) => draw_component_fin_set(draw_position, &component, parent),
        RocketComponent::Parachute(component) => draw_component_parachute(draw_position, &component, parent)
    }
}

/* -------------------------------------------------------------------------- */
/*                            Model Draw Functions                            */
/* -------------------------------------------------------------------------- */
// Outputs list of lines to draw for the entire model.
pub fn draw_model(model : &RocketModel) -> Vec<Vec<Vec2>> {
    // Draws all components in the given model sequentially, with each component having its subcomponents
    // drawn after the draw position is incremented for the next airframe section 
    // (making it so default position is at the end of the last section)
    let mut draw_position: f32 = 0.0;
    let mut output: Vec<Vec<Vec2>> = vec![];
    for section in &model.sections {
        output.append(&mut draw_section(draw_position, section)); 
        draw_position += section.length.value_true;
        for component in &section.components { output.append(&mut draw_component(draw_position, component, section))}
    }
    return output;
}

/* -------------------------------------------------------------------------- */
/*                                     UI                                     */
/* -------------------------------------------------------------------------- */
pub fn show(ui: &mut Ui, app: &mut App) {
    // Allocate painter and set clipping.
    let fb_rect: Rect = ui.available_rect_before_wrap();
    let (_response, mut painter) = ui.allocate_painter(Vec2::new(fb_rect.width(), fb_rect.height()), Sense::hover());
    painter.set_clip_rect(fb_rect.expand(1.0));

    // Draw backdrop.
    let rounding_config = CornerRadius::default().at_least(2);
    let stroke_config = Stroke::new(1.0, Color32::DARK_GRAY);
    let stroke_kind = StrokeKind::Middle;
    painter.rect(fb_rect, rounding_config, Color32::BLACK, stroke_config, stroke_kind);

    // Find points and total area of component lines.
    let model_lines: Vec<Vec<Vec2>> = draw_model(&mut app.model);
    let mut pos_min: Vec2 = vec2(0.0, 0.0);
    let mut pos_max: Vec2 = vec2(0.0, 0.0);
    for line in &model_lines {
        for point in line {
            pos_min = pos_min.min(*point);
            pos_max = pos_max.max(*point);
        }
    }

    // TODO Indicator lines.

    // Draw model lines, using total area to scale.
    let rocket_length: f32 = pos_max.y - pos_min.y;
    let scale: f32 = painter.clip_rect().height() / (rocket_length * 1.5);
    let center: Vec2 = painter.clip_rect().center().to_vec2() - vec2(0.0, scale * rocket_length / 2.0);
    for line in &model_lines {
        let stroke: Stroke = Stroke { width: 1.0, color: Color32::WHITE };
        painter.line(line.iter().map(|f| { (*f * scale + center).to_pos2() }).collect(), stroke);
    }

    // TODO Remove, test code : painter.add(Shape::dashed_line(&[pos2(0.0, 0.0), pos2(100.0, 100.0)], stroke_config, 5.0, 5.0));
}