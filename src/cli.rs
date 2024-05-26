// slint::include_modules!();
use std::error::Error;
// use std::io::{self, BufRead};
extern crate office_template_helper;
use office_template_helper::addon;
use office_template_helper::arguments;
use office_template_helper::file_handler;
use office_template_helper::modify;

// mod addon;
// mod arguments;
// mod file_handler; // Ensure the file name matches the module name, consider renaming it to file_handler.rs to follow Rust conventions
// mod modify;


fn main() -> Result<(), Box<dyn Error>> {
    
    println!(
        "
     ____  _                _     _____ _           _ _     _             
    / __ \\| |              | |   |_   _| |         | (_)   | |            
   | |  | | |__   ___  _ __| |_    | | | |__   __ _| |_ ___| |_ ___  _ __ 
   | |  | | '_ \\ / _ \\| '__| __|   | | | '_ \\ / _` | | / __| __/ _ \\| '__|
   | |__| | | | | (_) | |  | |_   _| |_| | | | (_| | | \\__ \\ || (_) | |   
    \\____/|_| |_|\\___/|_|   \\__| |_____|_| |_|\\__,_|_|_|___/\\__\\___/|_|   
    "
    );

    // Parse arguments using the CLI module

    let config = arguments::start();

    let arg_file = config.file;
    let arg_program = config.program;
    let arg_addon = config.addon;
    
    println!("Operating on file: {}", &arg_file);
    println!("Program: {}", &arg_program);
    println!("Addon: {}", &arg_addon);

    println!("Starting application...");

    // Example calls to functions in your modules
    let (sections_details, _ext) = match addon::initialize_addons() {
        Ok(data) => data,
        Err(e) => {
            // Log the error and exit if the initialization fails
            println!("Failed to initialize add-ons: {:?}", e);
            return Err(e);
        }
    };
    println!("Add-ons initialized.");

    // If initialization is successful, process the sections
    let _ = addon::print_sections(&sections_details, "all", None, true)?;
    // for section in &sections_details {
    //     print!("{}: ", section.name);
    //     for subsection in &section.subsections {
    //         if subsection.count > 1 {
    //             continue; // Skip this subsection because its count is greater than 1, suggesting it's a duplicate or should be ignored
    //         }
    //         print!("- {} ({})", subsection.name, subsection.count);
    //     }
    //     println!(); // End the line after listing subsections
    // }

    println!("_____________get file_______________");

    let normalized_base = arg_file.replace("\\", "/");
    let (ext, file) = file_handler::check_file(&normalized_base);
    println!("file path {}, has the File extension: {}", file, ext);

    println!("_____________Copy and verify file_______________");
    //copy the zip file to a new file
    let new_file = file_handler::copy_zip(&normalized_base).unwrap();
    println!("new file path: {}", new_file);

    // Open zip and read all file paths
    let entries = file_handler::open_zip(&file, false)?;
    let new_entries = file_handler::open_zip(&new_file, false)?;

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
            let ref_entries = file_handler::open_zip(&ref_zip, false)?;
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

    // get the addon snippet
    println!("_____________Get edits_______________");
    let program: &str = &arg_program;
    let addin: &str = &arg_addon;
    let edits = match addon::find_section_and_edits(program, addin, &sections_details) {
        Ok(edits) => edits,
        Err(e) => {
            println!("Error finding edits: {:?}", e);
            return Err(e);
        }
    };

    //modify the xml content
    println!("_____________modifying_______________");
    for edit in edits {
        println!("-----------------------------");
        let inner_path = edit.0;
        let changes = edit.1;
        let location = edit.2;
        let after = edit.3;
        println!("editing: {:?}: {}", inner_path, changes);
        println!("location: {} after {}", location, after);
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
        println!(
            "writing the content to the zip file: {} - {}",
            &new_file, &inner_path
        );
        file_handler::write_content_to_zip(&new_file, &inner_path, &res)?;
        println!("-----------------------------");
    }

    // Change the extension of the new file back to the original extension
    let original_filetype = file_handler::change_extension_to_original(&new_file, &ext);
    println!("Changed extension back to original: {}", original_filetype);
    println!("_________________________");

    println!(" Operation completed. Press `Enter` to exit");


    let _ = arguments::end_of_program();
    Ok(()) // Explicitly return Ok(()) to signify successful execution
}
