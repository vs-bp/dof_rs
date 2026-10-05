use elegance::egui::{ Ui, ScrollArea };
use crate::*;
use crate::units::*;
use crate::materials::*;
use crate::model::finset::*;
use crate::model::section::*;
use crate::workbench::ui_section::*;

mod ui_base;
mod ui_component;
mod ui_section;

/* -------------------------------------------------------------------------- */
/*                                   Update                                   */
/* -------------------------------------------------------------------------- */
// Recalculates automatic size values for current model.
pub fn update(app: &mut App) {
    // Log for what to set automatic tube sizes to.
    let mut diam_last: UnitValue = UnitValue { value_true: 0.0, value_ui: 0.0, unit: UnitType::Inch };
    for section in &mut app.model { 
        // Aft set to fore if not tapered.
        if !section.tapered {
            section.diameter_aft = section.diameter_fore;
        }
        // Diameters set to diam_last if set to automatic.
        if section.copy_diameter_aft { section.diameter_aft = diam_last; }
        if section.copy_diameter_fore { section.diameter_fore = diam_last; }
        // Diam_last set from aft diam.
        diam_last = section.diameter_aft;
    }
}

/* -------------------------------------------------------------------------- */
/*                                     UI                                     */
/* -------------------------------------------------------------------------- */


// Draws UI for modifying the current model.
pub fn show(ui: &mut Ui, app: &mut App) {
    ui.heading("Airframe Workbench");

    // Component add UI.
    ui.horizontal(|ui| {
        if ui.button("Add Section").clicked() { app.model.push(RocketSection::default()); }
    });
    
    // Component edit UI.
    // Reset actions to use directly after.
    app.last_workbench_action = WorkbenchAction::None;
    ScrollArea::new([false, true]).show(ui, |ui| {
        for section_idx in 0..app.model.len() { show_section(ui, section_idx, app); }
    });

    // Process last read component action. (Reorder or delete)
    match app.last_workbench_action {
        WorkbenchAction::None => {},
        WorkbenchAction::SwapSection(a, b) => { app.model.swap(a, b); }
        WorkbenchAction::DeleteSection(index) => { app.model.remove(index); }
        WorkbenchAction::SwapComponent(s, a, b) => { app.model[s].components.swap(a,b); }
        WorkbenchAction::DeleteComponent(s, idx) => { app.model[s].components.remove(idx); }
    }
    app.last_workbench_action = WorkbenchAction::None;
}