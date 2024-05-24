use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Deserialize, Debug)]
struct Config {
    // This will capture any top-level sections with their key-value pairs
    #[serde(flatten)]
    sections: HashMap<String, HashMap<String, String>>,
}

fn load_config(file_path: &str) -> Result<Config, Box<dyn Error>> {
    println!("Loading configuration from: {}", file_path);
    let config_contents = fs::read_to_string(file_path)?;
    let config: Config = toml::from_str(&config_contents)?;
    Ok(config)
}

//return a 2d array of the addon and extension sections
pub fn initialize_addons() -> Result<(), Box<dyn Error>> {
    println!("Add-ons initialized.");
    let addon = load_config("./settings/addon.toml")?;
    let extension = load_config("./settings/valid_file.toml")?;

    // println!("Addon Sections: {:?}", addon.sections);
    // println!("Extension Sections: {:?}", extension.sections);

    for (section, values) in addon.sections {
        println!(
            "Addon variable {} = {{{}}}",
            section,
            values.keys().cloned().collect::<Vec<_>>().join(", ")
        );
    }
    for (section, values) in extension.sections.clone() {
        println!(
            "Extension variable {} = {{{}}}",
            section,
            values.keys().cloned().collect::<Vec<_>>().join(", ")
        );
    }

    Ok(())
}

pub fn compare_zip_contents(
    ref_entries: &[(String, String)],
    test_entries: &[(String, String)],
) -> bool {
    // Convert vectors to hash maps for easy lookup
    let original_map: HashMap<&String, &String> = ref_entries
        .iter()
        .map(|(path, content)| (path, content))
        .collect();
    let new_map: HashMap<&String, &String> = test_entries
        .iter()
        .map(|(path, content)| (path, content))
        .collect();

    // Check if both archives have the same set of files
    if original_map.len() != new_map.len() {
        return false;
    }

    // Compare the content of each file
    for (path, original_content) in &original_map {
        if let Some(new_content) = new_map.get(path) {
            if original_content != new_content {
                return false; // Contents do not match
            }
        } else {
            return false; // File path does not exist in the new archive
        }
    }

    true // All files match
}

//compare file paths in a zip
pub fn compare_zip_files_path(
    ref_entries: &[(String, String)],
    test_entries: &[(String, String)],
) -> bool {
    // get the first element of the tuple of the enteries variable and compare the file paths
    let ref_paths: Vec<String> = ref_entries.iter().map(|(path, _)| path.clone()).collect();
    let test_paths: Vec<String> = test_entries.iter().map(|(path, _)| path.clone()).collect();

    //compare the paths of the ref and test zip files by looping through the paths i the ref_paths variable and test_paths variable
    for path in ref_paths {
        println!("\t  path: {}", path);
        if !test_paths.contains(&path) {
            return false;
        }
    }

    //if the path is not found in the test_paths variable return false else return true
    return true;
}

// fine the associated reference zip to compare
pub fn find_reference_zip(ext: &str) -> Result<String, Box<dyn Error>> {
    // to check for the path use the below code:
    let extension = load_config("./settings/valid_file.toml").unwrap();
    println!("looking for reference zip for '{}'", ext);
    if let Some(paths) = extension.sections.get("reference") {
        if let Some(refer_path) = paths.get(ext) {
            println!("reference zip for '{}': {:?}", ext, refer_path);
            return Ok(refer_path.clone());
        } else {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, format!("{} key not found", ext))))
        }
    } else {
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "paths section not found")))

    }
}

// find the associated test add snippet to deliver to the modifier
pub fn find_test_add_snippet(addon: &str) -> String {
    unimplemented!("find_test_add_snippet function not implemented yet");
    //let addon = load_config("./setting/addon.toml")?;
}
