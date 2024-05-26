
mod file_handler;
mod addon;
mod arguments;
mod modify;
use slint::SharedString;
mod gui_helper;
use std::error::Error;

pub mod ui {
    slint::include_modules!();
}

use ui::*;

fn main() -> Result<(), Box<dyn Error>> {
    let ui = Office::new().unwrap();


    // initalize the add-ons
    let (sections_details, _ext) = match addon::initialize_addons() {
        Ok(data) => data,
        Err(e) => {
            // Log the error and exit if the initialization fails
            let new_message = format!("Failed to initialize add-ons: {:?}", e);
            gui_helper::append_to_output_text(&ui, new_message);
            return Err(e);
        }
    };

    let new_message = format!("Add-ons initialized.");
    gui_helper::append_to_output_text(&ui, new_message);

    // If initialization is successful, process the sections
    for section in &sections_details {
        let new_message = format!("{}: ", section.name);
        gui_helper::append_to_output_text(&ui, new_message);
        for subsection in &section.subsections {
            let new_message = format!("- {} ({})", subsection.name, subsection.count);
            gui_helper::append_to_output_text(&ui, new_message);
        }
        //println!(); // End the line after listing subsections
    }

    gui_helper::append_to_output_text(&ui, String::from("_____________get file_______________"));
    ui.on_open_dialog({
        let ui_handle = ui.as_weak().clone();  // Clone the UI handle to move into the closure
        move || {
            let ui = ui_handle.upgrade().unwrap();
            if let Some(path) = gui_helper::show_dialog() {
                ui.set_file_path(SharedString::from(path.to_str().unwrap_or("Error reading path")));
                let new_message = format!("File selected: {}", path.display());
                gui_helper::append_to_output_text(&ui, new_message)
                // ui.set_output_text(SharedString::from(format!("File selected: {}", path.display()))); // Convert String to SharedString
            }
        }
    });

    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
        }
    });

    ui.run().unwrap();
    Ok(())
}
