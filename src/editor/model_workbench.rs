use elegance::egui::{ Ui };
use crate::*;
use crate::editor::ui_section::*;
use crate::editor::ui_component::*;

/* -------------------------------------------------------------------------- */
/*                                   Update                                   */
/* -------------------------------------------------------------------------- */
// Recalculates automatic size values for current model.
pub fn update(app: &mut App) {
    // Log for what to set automatic tube sizes to.
    let mut diam_last: UnitValue = UnitValue { value_true: 0.0, value_ui: 0.0, unit: UnitType::Inch };
    for section in &mut app.model.sections { 
        // Aft set to fore if not tapered.
        if !section.tapered {
            section.diameter_aft = section.diameter_fore;
        }
        // Diameters set to diam_last if set to automatic.
        if section.workbench_copy_diameter_aft { section.diameter_aft = diam_last; }
        if section.workbench_copy_diameter_fore { section.diameter_fore = diam_last; }
        // Diam_last set from aft diam.
        diam_last = section.diameter_aft;
    }
}

/* -------------------------------------------------------------------------- */
/*                                     UI                                     */
/* -------------------------------------------------------------------------- */


// Draws UI for modifying the current model.
pub fn show(ui: &mut Ui, app: &mut App) {
    ui.heading("Model Workbench");
    ui.separator();

    // Model properties UI.
    // TODO show_mass_properties(ui, 10000000, "Model Properties".to_string(), &mut app.model.int_mass_properties);

    // Show edit UI for given section.
    match app.workbench_selection {
        WorkbenchSelection::None => {},
        WorkbenchSelection::Section(s) => show_section(ui, s, app),
        WorkbenchSelection::Component(s, c) => show_component(ui, s, c, app)
    };

    // Model internals update which gets cascaded to all sections and components.
    app.model.update_internal();
}