// slint::include_modules!();

mod addon;
mod file_handler; // Ensure the file name matches the module name, consider renaming it to file_handler.rs to follow Rust conventions
mod modify;

fn main() -> Result<(), std::io::Error> {
    println!("Starting application...");

    // Example calls to functions in your modules
    let keys = addon::initialize_addons();
    if keys.is_ok() {
        println!("Add-ons initialized.");
    } else {
        println!("{:?}", keys.err().unwrap());
    }

    println!("____________________________");

    let normalized_base = "./test/simple.xyz".replace("\\", "/");
    let (ext, file) = file_handler::check_file(&normalized_base);
    println!("file path {}, has the File extension: {}", file, ext);

    println!("____________________________");
    //copy the zip file to a new file
    let new_file = file_handler::copy_zip(&normalized_base).unwrap();
    println!("new file path: {}", new_file);

    // Open zip and read all file paths and their contents
    let entries = file_handler::open_zip(&file, false)?;
    let new_entries = file_handler::open_zip(&new_file, false)?;

    // Compare the entries of the original zip file with the new zip file
    if addon::compare_zip_contents(&entries, &new_entries) {
        println!("The ZIP archives are identical.");
    } else {
        println!("The ZIP archives are not identical.");
    }

    // Compare the file paths of the original zip file with the new zip file
    match addon::find_reference_zip(&ext) {
        Ok(ref_zip) => {
            // Open zip and read all file paths and their contents
            let ref_entries = file_handler::open_zip(&ref_zip, false)?;
            if addon::compare_zip_files_path(&entries, &ref_entries) {
                println!("The ZIP archives have the same file paths.");
            } else {
                println!("The ZIP archives do not have the same file paths.");
            }
        }
        Err(e) => println!("Error finding reference zip: {:?}", e),
    }

    let full_path = file_handler::open_zip(&new_file, true)?;

    let file_content = &full_path[0].1;
    println!("file path: {}", &full_path[0].0);

    // get the addon snippet

    println!("____________________________");
    println!("modifying the xml content...");
    //modify the xml content
    let res = modify::modify_xml(&file_content)?;

    //if okay the print the result
    let formatted = modify::prettify_xml(&res)?;
    println!("formatted: {}", formatted);

    println!("____________________________");
    println!("writing the content to the zip file: {}", &full_path[0].0);
    file_handler::write_content_to_zip(&new_file, &new_entries[0].0, &res)?;

    // Change the extension of the new file back to the original extension
    let original_ext = "abc";
    let original_filetype = file_handler::change_extension_to_original(&new_file, original_ext);
    println!("Changed extension back to original: {}", original_filetype);

    // println!(" Operation completed.");
    Ok(()) // Explicitly return Ok(()) to signify successful execution
}

// fn main() -> Result<(), slint::PlatformError> {
//     let ui = AppWindow::new()?;

//     ui.on_request_increase_value({
//         let ui_handle = ui.as_weak();
//         move || {
//             let ui = ui_handle.unwrap();
//             ui.set_counter(ui.get_counter() + 1);
//         }
//     });

//     ui.run()
// }
