extern crate office_template_helper;
use office_template_helper::addon;

use slint::{ModelRc, SharedString};
mod gui_helper;
use slint::VecModel;
use std::error::Error;
use ui::*;

pub mod ui {
    slint::include_modules!();
}


fn main() -> Result<(), Box<dyn Error>> {
    let ui = Office::new().unwrap();

    // initalize the add-ons
    let (sections_details, _ext) = match addon::initialize_addons() {
        Ok(data) => data,
        Err(e) => {
            // Log the error and exit if the initialization fails
            let new_message = format!("Failed to initialize add-ons: {:?}", e);
            gui_helper::append_to_output_text(&ui, &new_message);
            return Err(e);
        }
    };

    // If initialization is successful, process the sections
    // populate the sections (programs) in the GUI drop down
    let programs = addon::print_sections(&sections_details, "section", None, false)?;
    let mut program_select = Vec::new();
    for program in programs {
        program_select.push(SharedString::from(program));
    }
    let programs_model = VecModel::from(program_select);
    let programs_model_rc = ModelRc::new(programs_model);
    ui.set_programs(programs_model_rc.clone());

    // set the addon combo box
    // wait for selection from program then populate the addon accordingly
    let section_details_clone = sections_details.clone();
    ui.on_program_selected({
        let ui_handle = ui.as_weak().clone();
        move |program| {
            println!("Program selected: {}", program);
            let ui = ui_handle.upgrade().unwrap();
            let selected_program = program.as_str();
            let addons = addon::print_sections(
                &section_details_clone,
                "subsection",
                Some(selected_program),
                false,
            )
            .unwrap();
            let mut addon_select = Vec::new();
            for addon in addons {
                addon_select.push(SharedString::from(addon));
            }
            let addons_model = VecModel::from(addon_select);
            let addons_model_rc = ModelRc::new(addons_model);
            ui.set_addons(addons_model_rc.clone());
        }
    });
    // get the add on call back and set the value on the args.addon using the on_addon_selected callback
    // this will print to the console the selected addon not the program
    ui.on_addon_selected({
        move |addon| {
            let selected_addon = addon.as_str();
            println!("Addon selected: {}", selected_addon);
        }
    });

    // this it to get the file path
    ui.on_open_dialog({
        let ui_handle = ui.as_weak().clone(); // Clone the UI handle to move into the closure
        move || {
            let ui = ui_handle.upgrade().unwrap();
            if let Some(path) = gui_helper::show_dialog() {
                ui.set_file_path(SharedString::from(
                    path.to_str().unwrap_or("Error reading path"),
                ));
                let new_message = format!("File selected: {}", path.display());
                gui_helper::append_to_output_text(&ui, &new_message)
                // ui.set_output_text(SharedString::from(format!("File selected: {}", path.display()))); // Convert String to SharedString
            }
        }
    });

    // this is to start the process
    // it will be assigned the start_prg callback and call the gui_helper::start function
    // the start function will take in the ui, file path, program name, and the addon name that will be  mapped into the callback
    let section_details_clone = sections_details.clone();
    ui.on_start_prg({
        let ui_handle = ui.as_weak().clone();
        move |file, pro, add| {
            let ui = ui_handle.upgrade().unwrap();
            let file_path = file.as_str();
            let program = pro.as_str();
            let addon = add.as_str();
            let new_message = format!(
                "Starting process with file: {}, program: {}, addon: {}",
                file_path, program, addon
            );
            gui_helper::append_to_output_text(&ui, &new_message);
            let _ = gui_helper::start(&ui, file_path, program, addon, &section_details_clone);
        }
    });

    // gui_helper::start(&ui, file_path, program, addon);

    ui.run().unwrap();
    Ok(())
}
