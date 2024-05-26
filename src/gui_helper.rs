use crate::ui::*;
use office_template_helper::addon;
use office_template_helper::addon::SectionDetail;
use office_template_helper::file_handler;
use office_template_helper::modify;
use slint::SharedString;
use std::path::PathBuf;

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
    println!("{}", new_message);
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
    let normalized_base = file_path.replace("\\", "/");
    let (ext, file) = file_handler::check_file(&normalized_base);
    println!("file path {}, has the File extension: {}", file, ext);
    (file, ext)
}

//copy the file and verify the file, i need the file path and it will return the new file path
fn copy_and_verify_file(file_path: &str, ext: &str) -> String {
    //copy the zip file to a new file
    let new_file = file_handler::copy_zip(&file_path).unwrap();
    println!("new file path: {}", new_file);

    // Open zip and read all file paths
    let entries = file_handler::open_zip(&file_path, false).unwrap();
    let new_entries = file_handler::open_zip(&new_file, false).unwrap();

    // Compare the entries of the original zip file with the new zip file
    if addon::compare_zip_files_path(&entries, &new_entries) {
        println!("The ZIP archives are identical.");
    } else {
        println!("The ZIP archives are not identical.");
    }

    // Compare the file paths of the original zip file with the ref zip file
    match addon::find_reference_zip(&ext) {
        Ok(ref_zip) => {
            // Open zip and read all file paths
            let ref_entries = file_handler::open_zip(&ref_zip, false).unwrap();
            if addon::compare_zip_files_path(&ref_entries, &entries) {
                println!("The ZIP archives have the same file paths.");
            } else {
                println!("The ZIP archives do not have the same file paths.");
            }
        }
        Err(e) => println!("Error finding reference zip: {:?}", e),
    }

    //print the files in the zip
    for entry in &new_entries {
        println!("file: {}", entry);
    }
    new_file
}

// public function to start the process it takes in the ui, file path,  program name, and the addon name
// it will return the a bool of success or fail with error message
pub fn start(
    ui: &Office,
    file_path: &str,
    program: &str,
    addon: &str,
    sections_details: &Vec<SectionDetail>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        " the arguemtns are: file: {}, program: {}, addon: {}",
        file_path, program, addon
    );
    append_to_output_text(&ui, "_____________get file_______________");
    let (file, ext) = get_file_name_and_extension(file_path);
    append_to_output_text(
        ui,
        format!("file path {}, has the File extension: {}", file, ext).as_str(),
    );
    append_to_output_text(&ui, "_____________Copy and verify file_______________");
    let new_file = copy_and_verify_file(file_path, &ext);
    append_to_output_text(&ui, format!("new file path: {}", new_file).as_str());

    append_to_output_text(&ui, "_____________Get edits_______________");
    let edits = match addon::find_section_and_edits(program, addon, &sections_details) {
        Ok(edits) => edits,
        Err(e) => {
            println!("Error finding edits: {:?}", e);
            return Err(e);
        }
    };
    append_to_output_text(&ui, format!("Number of edits: {:?}", edits.len()).as_str());

    // //modify the xml content
    //modify the xml content
    append_to_output_text(&ui, "_____________modifying_______________");
    for edit in edits {
        println!("-----------------------------");
        let inner_path = edit.0;
        let changes = edit.1;
        let location = edit.2;
        let after = edit.3;
        append_to_output_text(
            &ui,
            format!("editing: {:?}: {}", inner_path, changes).as_str(),
        );
        append_to_output_text(
            &ui,
            format!("location: {} after {}", location, after).as_str(),
        );
        let file_content = file_handler::read_zip_file_content(&new_file, &inner_path)?;
        let file_content_str = std::str::from_utf8(&file_content)?.to_string(); // Convert &str to String
        let mut formatted = modify::prettify_xml(&file_content_str)?;
        println!("file content: {}", formatted);
        println!("-------");
        let res = modify::modify_xml(&file_content_str, &changes, &location, after)?; // Pass String as reference
                                                                                      //if okay the print the result
        formatted = modify::prettify_xml(&res)?;
        println!("formatted: {}", formatted);
        println!("____________________________");
        append_to_output_text(
            &ui,
            format!(
                "writing the content to the zip file: {} - {}",
                &new_file, &inner_path
            )
            .as_str(),
        );
        file_handler::write_content_to_zip(&new_file, &inner_path, &res)?;
        println!("-----------------------------");
    }

    append_to_output_text(&ui, "==================================");
    // Change the extension of the new file back to the original extension
    let original_filetype = file_handler::change_extension_to_modified(&new_file, &file, &ext);
    append_to_output_text(
        &ui,
        format!("Changed extension back to original: {}", original_filetype).as_str(),
    );

    Ok(())
}
