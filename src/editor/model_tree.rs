use std::io::{Read, Write};

use elegance::egui::{ Ui, ComboBox };
use crate::{model::{component_generic_mass::RocketComponentGenericMass, component_motor_mount::RocketComponentMotorMount}, *};

// Show heirarchy viewer and selector for all parts.
pub fn show(ui: &mut Ui, app: &mut App) {
    // Show save/load file menu.
    ui.horizontal(|ui| {
        ui.label("Model");
        if ui.button("Save").clicked() { 
            let file_path = rfd::FileDialog::new().add_filter("dof_rs Rocket", &["drf"]).save_file(); 
            if file_path.is_some() {
                let mut file: std::fs::File = std::fs::File::create(file_path.unwrap()).unwrap();
                file.write(&app.model.as_bytes()).unwrap(); 
            }
        }
        if ui.button("Load").clicked() { 
            let file_path = rfd::FileDialog::new().add_filter("dof_rs Rocket", &["drf"]).pick_file();
            if file_path.is_some() {
                let file: std::fs::File = std::fs::File::open(file_path.unwrap()).unwrap();
                app.model = RocketModel::from_bytes(file.bytes().map(|f| f.unwrap()).collect());
                app.workbench_selection = WorkbenchSelection::None;
            }
        }
    });

    // Show addition + contextual move/delete menu.
    // Actions here create new workbench_actions which are handled after the menu is done drawing.
    let mut action: WorkbenchAction = WorkbenchAction::None;
    // Contextual action based off selection.
    match app.workbench_selection {
        WorkbenchSelection::None => {
            ui.horizontal(|ui| {
                ui.label("Section");
                if ui.button("+").clicked() { app.model.sections.push(RocketSection::new()); }
                if ui.button("↑").clicked() {}
                if ui.button("↓").clicked() {}
                if ui.button("x").clicked() {}
            });
            ui.horizontal(|ui| {
                ui.label("Component");
                if ui.button("+").clicked() {}
                if ui.button("↑").clicked() {}
                if ui.button("↓").clicked() {}
                if ui.button("x").clicked() {}
            });
        },
        WorkbenchSelection::Section(s) => {
            ui.horizontal(|ui| {
                ui.label("Section");
                if ui.button("+").clicked() { app.model.sections.push(RocketSection::new()); }
                if ui.button("↑").clicked() { if s > 0 { action = WorkbenchAction::SwapSection(s, s-1); }}
                if ui.button("↓").clicked() { if s < app.model.sections.len() - 1 { action = WorkbenchAction::SwapSection(s, s+1); }}
                if ui.button("x").clicked() { action = WorkbenchAction::DeleteSection(s); }
            });
            ui.horizontal(|ui| {
                ui.label("Component");              
                let mut addition_type: usize = 0;
                ComboBox::new("modeltreeaddcomponent", "").width(20.0).selected_text("+").show_ui(ui, |ui| {
                    ui.selectable_value(&mut addition_type, 1, "Add Fin Set");
                    ui.selectable_value(&mut addition_type,2, "Add Generic Mass");
                    ui.selectable_value(&mut addition_type,3, "Add Motor Mount");
                });
                match addition_type {
                    1 => { app.model.sections[s].components.push(RocketComponent::FinSet(RocketComponentFinSet::default())); },
                    2 => { app.model.sections[s].components.push(RocketComponent::GenericMass(RocketComponentGenericMass::default())); },
                    3 => { app.model.sections[s].components.push(RocketComponent::MotorMount(RocketComponentMotorMount::default())); },
                    _ => {}
                }
                if ui.button("↑").clicked() {}
                if ui.button("↓").clicked() {}
                if ui.button("x").clicked() {}
            });
        },
        WorkbenchSelection::Component(s, c) => {
            ui.horizontal(|ui| {
                ui.label("Section");
                if ui.button("+").clicked() { app.model.sections.push(RocketSection::new()); }
                if ui.button("↑").clicked() { if s > 0 { action = WorkbenchAction::SwapSection(s, s-1); }}
                if ui.button("↓").clicked() { if s < app.model.sections.len() - 1 { action = WorkbenchAction::SwapSection(s, s+1); }}
                if ui.button("x").clicked() { action = WorkbenchAction::DeleteSection(s); }
            });
            ui.horizontal(|ui| {
                ui.label("Component");
                let mut addition_type: usize = 0;
                ComboBox::new("modeltreeaddcomponent", "").width(20.0).selected_text("+").show_ui(ui, |ui| {
                    ui.selectable_value(&mut addition_type, 1, "Add Fin Set");
                    ui.selectable_value(&mut addition_type,2, "Add Generic Mass");
                    ui.selectable_value(&mut addition_type,3, "Add Motor Mount");
                });
                match addition_type {
                    1 => { app.model.sections[s].components.push(RocketComponent::FinSet(RocketComponentFinSet::default())); },
                    2 => { app.model.sections[s].components.push(RocketComponent::GenericMass(RocketComponentGenericMass::default())); },
                    3 => { app.model.sections[s].components.push(RocketComponent::MotorMount(RocketComponentMotorMount::default())); },
                    _ => {}
                }
                if ui.button("↑").clicked() { if c > 0 { action = WorkbenchAction::SwapComponent(s, c, c-1); }}
                if ui.button("↓").clicked() { if c < app.model.sections.len() - 1 { action = WorkbenchAction::SwapComponent(s, c, c-1); }}
                if ui.button("x").clicked() { action = WorkbenchAction::DeleteComponent(s, c); }
            });
        }
    }

    // Process workbench actions on draw complete.
    match action {
        WorkbenchAction::None => {},
        WorkbenchAction::SwapSection(a, b) => { 
            app.model.sections.swap(a, b);
            app.workbench_selection = WorkbenchSelection::Section(b);
        }
        WorkbenchAction::DeleteSection(index) => { 
            app.model.sections.remove(index); 
            app.workbench_selection = WorkbenchSelection::None;
        }
        WorkbenchAction::SwapComponent(s, a, b) => { 
            app.model.sections[s].components.swap(a,b); 
            app.workbench_selection = WorkbenchSelection::Component(s, b);
        }
        WorkbenchAction::DeleteComponent(s, idx) => { 
            app.model.sections[s].components.remove(idx); 
            app.workbench_selection = WorkbenchSelection::None;
        }
    }

    ui.separator();

    // Display sections as a list of labels.
    for section_idx in 0..app.model.sections.len() {
        // Label based off of section dimensions.
        let mut text: &str = "Body Tube";
        if app.model.sections[section_idx].tapered { text = "Transition"; }
        if app.model.sections[section_idx].diameter_fore.value_true == 0.0 { text = "Nose Cone"; }
        // Whether the label is highlighted is determined by the workbench selection enum stored in app state.
        let checked: bool = match app.workbench_selection {
            WorkbenchSelection::Section(s) => { s == section_idx },
            _ => false
        };
        // Workbench selection enum can be overwritten on click.
        // Override style to make labels shorter.
        ui.style_mut().spacing.button_padding.y = 0.0;
        if ui.selectable_label(checked, text).clicked() { app.workbench_selection = WorkbenchSelection::Section(section_idx); }
        // Draw components below selection with separators.
        if app.model.sections[section_idx].components.len() > 0 {
            let mut component_idx: usize = 0;
            for generic_component in &mut app.model.sections[section_idx].components {
                ui.horizontal(|ui| {
                    // Keep above style.
                    ui.style_mut().spacing.button_padding.y = 0.0;
                    ui.separator();
                    let text: &str = match generic_component {
                        RocketComponent::FinSet(_) => "Fin Set",
                        RocketComponent::GenericMass(_) => "Generic Mass",
                        RocketComponent::MotorMount(_) => "Motor Mount"
                    };
                    let checked:bool = match app.workbench_selection {
                        WorkbenchSelection::Component(s, c) => { (s == section_idx) && (c == component_idx) },
                        _ => false
                    };
                    if ui.selectable_label(checked, text).clicked() { app.workbench_selection = WorkbenchSelection::Component(section_idx, component_idx); }
                    component_idx += 1;
                });
            }
        }
    }
}