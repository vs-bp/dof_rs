use core::f32;
use elegance::egui::{ Vec2, vec2, Ui, Rect, Sense, CornerRadius, Stroke, Color32, StrokeKind };
use crate::App;

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
    let model_lines: Vec<Vec<Vec2>> = app.model.draw();
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