use std::path::PathBuf;
use slint::SharedString;
use crate::ui::*;
use office_template_helper::arguments;
use office_template_helper::file_handler;
use office_template_helper::modify;

// use slint::*;


pub fn show_dialog() -> Option<PathBuf> {
    let current_dir = std::env::current_dir().unwrap();
    let path = rfd::FileDialog::new()
        .set_title("Select file")
        .set_directory(current_dir)
        .pick_file();

    path
}

pub fn append_to_output_text(ui: &Office, new_message: &str) {
    // Get the current content of output_text
    let current_text = ui.get_output_text();
    // Append the new message
    let new_text = SharedString::from(std::format!("{}\n{}", current_text, new_message));
    // Update the output_text property
    ui.set_output_text(SharedString::from(new_text));
}


// get the file name and extension and normalize the path
// the function will take in the string of the path and return the string string of the file name and the extension
fn get_file_name_and_extension(file_path: &str) -> (String, String) {
    println!("_____________get file_______________");

    let normalized_base = file_path.replace("\\", "/");
    let (ext, file) = file_handler::check_file(&normalized_base);
    println!("file path {}, has the File extension: {}", file, ext);
    (file, ext)
}

// public function to start the process it takes in the ui, file path,  program name, and the addon name
// it will return the a bool of success or fail with error message
pub fn start(ui: &Office, file_path: &str, program: &str, addon: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!(" the arguemtns are: file: {}, program: {}, addon: {}", file_path, program, addon);
    let (file, ext) = get_file_name_and_extension(file_path);
    print!("file: {}, ext: {}", file, ext);


    // println!("_____________Copy and verify file_______________");
    // //copy the zip file to a new file
    // let new_file = file_handler::copy_zip(&file).unwrap();
    // println!("new file path: {}", new_file);

    // // Open zip and read all file paths
    // let entries = file_handler::open_zip(&file, false)?;
    // let new_entries = file_handler::open_zip(&new_file, false)?;

    // // Compare the entries of the original zip file with the new zip file
    // if modify::compare_zip_files_path(&entries, &new_entries) {
    //     println!("The ZIP archives are identical.");
    // } else {
    //     println!("The ZIP archives are not identical.");
    // }

    // // Compare the file paths of the original zip file with the ref zip file
    // match modify::find_reference_zip(&ext) {
    //     Ok(ref_zip) => {
    //         // Open zip and read all file paths
    //         let ref_entries = file_handler::open_zip(&ref_zip, false)?;
    //         if modify::compare_zip_files_path(&ref_entries, &entries) {
    //             println!("The ZIP archives have the same file paths.");
    //         } else {
    //             println!("The ZIP archives do not have the same file paths.");
    //         }
    //     }
    //     Err(e) => println!("Error finding reference zip: {:?}", e),
    // }

    // //print the files in the zip
    // for entry in &new_entries {
    //     println!("file: {}", entry);
    // }

    // // get the addon snippet
    // println!("_____________Get edits_______________");
    // let edits = match modify::find_section_and_edits(program, addon) {
    //     Ok(edits) => edits,
    //     Err(e) => {
    //         println!("Error finding edits: {:?}", e);
    //         return Err(e);
    //     }
    // };

    // //modify the xml content
    // modify::modify_xml_content(&new_file, &edits)?;

    Ok(())
}