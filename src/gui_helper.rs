use std::path::PathBuf;
use slint::SharedString;
use crate::ui::*;
// use slint::*;


pub fn show_dialog() -> Option<PathBuf> {
    let current_dir = std::env::current_dir().unwrap();
    let path = rfd::FileDialog::new()
        .set_title("Select file")
        .set_directory(current_dir)
        .pick_file();

    path
}

pub fn append_to_output_text(ui: &Office, new_message: String) {
    // Get the current content of output_text
    let current_text = ui.get_output_text();
    // Append the new message
    let new_text = SharedString::from(std::format!("{}\n{}", current_text, new_message));
    // Update the output_text property
    ui.set_output_text(SharedString::from(new_text));
}