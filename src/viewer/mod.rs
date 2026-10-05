use core::f32;
use elegance::egui::{ Vec2, vec2, Ui, Rect, Sense, CornerRadius, Stroke, Color32, StrokeKind, Pos2 };
use crate::model::finset::*;
use crate::model::section::*;
use crate::App;

/* -------------------------------------------------------------------------- */
/*                            Section Draw Functions                          */
/* -------------------------------------------------------------------------- */
// Returns the vector of vec2 pairs which define the lines for the given section.
// Changes draw_position by the length of the section to define the starting position of the next section.
// Fins use draw_position but dont change it.
//
// draw_section_airframe serves as entry point, calling subroutines below.
pub fn draw_section_conical(draw_position: f32, section: &RocketSection) -> Vec<(Vec2, Vec2)> {
    let c_fore_low: Vec2 = vec2(-section.diameter_fore.value_true / 2.0, draw_position, );
    let c_fore_high: Vec2 = vec2(section.diameter_fore.value_true / 2.0, draw_position);
    let c_aft_low: Vec2 = vec2(-section.diameter_aft.value_true / 2.0, draw_position + section.length.value_true);
    let c_aft_high: Vec2 = vec2(section.diameter_aft.value_true / 2.0, draw_position + section.length.value_true);
    return vec![
        (c_fore_low, c_fore_high),
        (c_aft_low, c_aft_high),
        (c_fore_low, c_aft_low),
        (c_fore_high, c_aft_high)
    ]
}
// If not aft tangent, then curve will be tangent to tip.
pub fn draw_section_ogive(draw_position: f32, section: &RocketSection, aft_tangency: bool) -> Vec<(Vec2, Vec2)> {
    // Variable renaming for ease of reading.
    let d_fore: f32 = section.diameter_fore.value_true;
    let d_aft: f32 = section.diameter_aft.value_true;
    let r_fore: f32 = section.diameter_fore.value_true / 2.0;
    let r_aft: f32 = section.diameter_aft.value_true / 2.0;
    let l: f32 = section.length.value_true;

    // Early exit to conical case if the diameters match.
    // Ogive draw function results in infinities otherwise due to divisions by zero.
    if d_fore == d_aft { return draw_section_conical(draw_position, section); }

    // Determine ogive curve params.
    // Curve points and radii shifts with tangency definition.
    let center_y: f32 = 
        if aft_tangency { (r_aft * r_aft - r_fore * r_fore - l * l) / (d_aft - d_fore) }
        else { (r_fore * r_fore - r_aft * r_aft - l * l) / (d_fore - d_aft) };
    let center_x: f32 = 
        if aft_tangency { l } 
        else { 0.0 };
    let radius: f32 = r_aft - ((r_aft*r_aft - r_fore*r_fore - l*l)/(d_aft - d_fore));

    // Determine range and resolution of swept curve.
    const CURVE_POINTS: u32 = 32;
    let t_range: f32 = (l / radius).asin();
    let t0: f32 = f32::min(t_range, 0.0);
    let t1: f32 = f32::max(t_range, 0.0);
    let sign: f32 = if r_aft > r_fore { 1.0 } else { -1.0 };

    // Endcap points.
    let mut points: Vec<(Vec2, Vec2)> = vec![];
    points.push((
        vec2(0.0, draw_position) + vec2(r_fore, 0.0),
        vec2(0.0, draw_position) + vec2(-r_fore,0.0),
    ));
    points.push((
        vec2(0.0, draw_position) + vec2(r_aft,  l),
        vec2(0.0, draw_position) + vec2(-r_aft, l),
    ));

    // Swept curve points.
    for i in 0..CURVE_POINTS {
        let tl: f32 = (t1 - t0)*((i + 0) as f32 / CURVE_POINTS as f32); // current curve sweep parameter
        let th: f32 = (t1 - t0)*((i + 1) as f32 / CURVE_POINTS as f32); // next curve sweep parameter
        if aft_tangency {
            points.push((
                vec2(0.0, draw_position) + vec2(center_y + radius * tl.cos(), l - sign * radius * tl.sin()),
                vec2(0.0, draw_position) + vec2(center_y + radius * th.cos(), l - sign * radius * th.sin()),
            ));
            points.push((
                vec2(0.0, draw_position) + vec2(-center_y - radius * tl.cos(), l - sign * radius * tl.sin()),
                vec2(0.0, draw_position) + vec2(-center_y - radius * th.cos(), l - sign * radius * th.sin()),
            ));
        } else {
            points.push((
                vec2(0.0, draw_position) + vec2(center_y - radius * tl.cos(), sign * radius * tl.sin()),
                vec2(0.0, draw_position) + vec2(center_y - radius * th.cos(), sign * radius * th.sin()),
            ));
            points.push((
                vec2(0.0, draw_position) + vec2(-center_y + radius * tl.cos(), sign * radius * tl.sin()),
                vec2(0.0, draw_position) + vec2(-center_y + radius * th.cos(), sign * radius * th.sin()),
            ));
        }
    }
    return points;
}
pub fn draw_section(draw_position: f32, section: &RocketSection) -> Vec<(Vec2, Vec2)> {
    // Match curve type.
    // TODO More curve types pending.
    match &section.curve {
        RocketSectionCurveSpec::Conical => draw_section_conical(draw_position, section),
        RocketSectionCurveSpec::OgiveAft => draw_section_ogive(draw_position, section, true),
        RocketSectionCurveSpec::OgiveFore => draw_section_ogive(draw_position, section, false),
    }
}

/* -------------------------------------------------------------------------- */
/*                          Component Draw Functions                          */
/* -------------------------------------------------------------------------- */
// Returns the vector of vec2 pairs which define the lines for the given component.
// Fins use draw_position but dont change it.
pub fn draw_component_fin(draw_position: f32, angle: f32, body_diameter: f32, component: &RocketComponentFinSet) -> Vec<(Vec2, Vec2)> {
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
    return vec![
        (c_root_low, c_tip_low),
        (c_root_high, c_tip_high),
        (c_tip_high, c_tip_low)
    ];
}
pub fn draw_component_fin_set(draw_position: f32, component: &RocketComponentFinSet, parent: &RocketSection) -> Vec<(Vec2, Vec2)> {
    let mut points: Vec<(Vec2, Vec2)> = vec![];
    for i in 0..component.count {
        points.append(&mut draw_component_fin(draw_position, i as f32 * 2.0 * f32::consts::PI / component.count as f32, parent.diameter_aft.value_true, component));
    }
    return points;
}
pub fn draw_component_parachute(draw_position: f32, component: &RocketComponentParachute, parent: &RocketSection) -> Vec<(Vec2, Vec2)> {
    // TODO Variable color?

    // Basic coordinates.
    let c1: Vec2 = vec2(-component.packed_diameter.value_true / 2.0, -component.position_offset.value_true + draw_position);
    let c2: Vec2 = vec2(component.packed_diameter.value_true / 2.0, -component.position_offset.value_true + draw_position);
    let c3: Vec2 = vec2(component.packed_diameter.value_true / 2.0, -component.position_offset.value_true + draw_position - component.packed_length.value_true);
    let c4: Vec2 = vec2(-component.packed_diameter.value_true / 2.0, -component.position_offset.value_true + draw_position - component.packed_length.value_true);

    // Output.
    return vec![
        (c1, c2),
        (c2, c3),
        (c3, c4),
        (c4, c1)
    ];
}
// Generic function to draw any given component based off its type.
pub fn draw_component(draw_position: f32, generic_component: &RocketComponent, parent: &RocketSection) -> Vec<(Vec2, Vec2)> {
    match generic_component {
        RocketComponent::FinSet(component) => draw_component_fin_set(draw_position, &component, parent),
        RocketComponent::Parachute(component) => draw_component_parachute(draw_position, &component, parent)
    }
}

/* -------------------------------------------------------------------------- */
/*                            Model Draw Functions                            */
/* -------------------------------------------------------------------------- */
// Outputs list of lines to draw for the entire model.
pub fn draw_model(model : &Vec<RocketSection>) -> Vec<(Vec2, Vec2)> {
    // Draws all components in the given model sequentially, with each component having its subcomponents
    // drawn after the draw position is incremented for the next airframe section 
    // (making it so default position is at the end of the last section)
    let mut draw_position: f32 = 0.0;
    let mut output: Vec<(Vec2, Vec2)> = vec![];
    for section in model {
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
    let model_lines: Vec<(Vec2, Vec2)> = draw_model(&mut app.model);
    let mut pos_min: Vec2 = vec2(0.0, 0.0);
    let mut pos_max: Vec2 = vec2(0.0, 0.0);
    for line in &model_lines {
        pos_min = pos_min.min(line.0);
        pos_min = pos_min.min(line.1);
        pos_max = pos_max.max(line.0);
        pos_max = pos_max.max(line.1);
    }

    // TODO Indicator lines.

    // Draw model lines, using total area to scale.
    let rocket_length: f32 = pos_max.y - pos_min.y;
    let scale: f32 = painter.clip_rect().height() / (rocket_length * 1.5);
    let center: Vec2 = painter.clip_rect().center().to_vec2() - vec2(0.0, scale * rocket_length / 2.0);
    for line in &model_lines {
        let stroke: Stroke = Stroke { width: 1.0, color: Color32::WHITE };
        let points: Vec<Pos2> = vec![
            ((line.0 * scale) + center).to_pos2(), 
            ((line.1 * scale) + center).to_pos2()
        ];
        painter.line(points, stroke);
    }

    // TODO Remove, test code : painter.add(Shape::dashed_line(&[pos2(0.0, 0.0), pos2(100.0, 100.0)], stroke_config, 5.0, 5.0));
}