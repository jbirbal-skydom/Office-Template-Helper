
mod file_handler;
mod addon;
mod arguments;
mod modify;
use slint::SharedString;
mod gui_helper;

pub mod ui {
    slint::include_modules!();
}

use ui::*;

fn main() -> Result<(), slint::PlatformError> {
    // let matches = App::new("slint")
    //     .version("0.1.0")
    //     .author("Your Name <
    let ui = Office::new()?;

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

    ui.run()
}
