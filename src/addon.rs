use serde_yaml::{from_reader, Value};
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;

// Define a structure to hold the overall section details
#[derive(Clone)] // This line will auto-implement the Clone trait
pub struct SectionDetail {
    pub name: String,
    pub subsections: Vec<SubsectionDetail>,
}
// Define a structure to hold the details of each subsection
#[derive(Clone)] // This line will auto-implement the Clone trait
pub struct SubsectionDetail {
    pub name: String,
    pub file: String,
    pub edit: String,
    pub count: usize,
    pub loc: String,
    pub after: bool,
}

fn load_config(file_path: &str) -> Result<HashMap<String, Value>, Box<dyn Error>> {
    println!("Loading configuration from: {}", file_path);
    let file = File::open(file_path)?;
    let config: HashMap<String, Value> = from_reader(file)?;
    Ok(config)
}

//return a 2d array of the addon and extension sections
pub fn initialize_addons() -> Result<(Vec<SectionDetail>, HashMap<String, Value>), Box<dyn Error>> {
    println!("Add-ons initialized.");
    let addon = load_config("./settings/addon.yaml")?;
    let extension = load_config("./settings/valid_file.yaml")?;

    let sections_details = collect_sections_and_counts(&addon);

    Ok((sections_details, extension))
}

fn collect_sections_and_counts(config: &HashMap<String, Value>) -> Vec<SectionDetail> {
    let mut details: Vec<SectionDetail> = Vec::new();

    for (section_name, section_value) in config {
        let mut section_detail = SectionDetail {
            name: section_name.clone(),
            subsections: Vec::new(),
        };

        if let Value::Mapping(subsections) = section_value {
            for (sub_name, sub_value) in subsections {
                if let Value::Sequence(entries) = sub_value {
                    let count = entries.len(); // Correct counting here
                    for entry in entries {
                        if let Value::Mapping(entry_details) = entry {
                            let file = entry_details
                                .get("file")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string();
                            let edit = entry_details
                                .get("edits")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string();
                            let loc = entry_details
                                .get("loc")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string();
                            let after = entry_details
                                .get("after")
                                .and_then(|v| v.as_bool())
                                .unwrap_or(false);
                            section_detail.subsections.push(SubsectionDetail {
                                name: sub_name.as_str().unwrap_or("unknown").to_string(),
                                file,
                                edit,
                                count,
                                loc,
                                after,
                            });
                        }
                    }
                } else {
                    // Handle case where the structure is not as expected
                    section_detail.subsections.push(SubsectionDetail {
                        name: sub_name.as_str().unwrap_or("unknown").to_string(),
                        file: String::new(),
                        edit: String::new(),
                        loc: String::new(),
                        after: false,
                        count: 0,
                    });
                }
            }
        }

        details.push(section_detail);
    }

    details
}
//compare file paths in a zip
pub fn compare_zip_files_path(ref_entries: &[String], test_entries: &[String]) -> bool {
    let ref_set: HashSet<_> = ref_entries.iter().collect();
    let test_set: HashSet<_> = test_entries.iter().collect();

    // Check if both sets contain the same elements
    if ref_set == test_set {
        true
    } else {
        // Optionally, you can identify which paths are missing in each set
        let missing_in_ref = test_set.difference(&ref_set);
        let missing_in_test = ref_set.difference(&test_set);

        for path in missing_in_ref {
            println!("\tMissing in reference: {}", path);
        }

        for path in missing_in_test {
            println!("\tMissing in test: {}", path);
        }

        false
    }
}

// fine the associated reference zip to compare
pub fn find_reference_zip(ext: &str) -> Result<String, Box<dyn Error>> {
    println!("Looking for reference zip for '{}'", ext);
    let config = load_config("./settings/valid_file.yaml")?;

    // Assume 'reference' is a top-level key in the YAML that maps to a dictionary
    if let Some(Value::Mapping(reference_map)) = config.get("reference") {
        if let Some(Value::String(refer_path)) = reference_map.get(&Value::String(ext.to_string()))
        {
            println!("Reference zip for '{}': {}", ext, refer_path);
            Ok(refer_path.clone())
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("{} key not found", ext),
            )))
        }
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "reference section not found",
        )))
    }
}

// find the section and the amount of edits and which file to edit
// the function will take in the zip path,the add-on name, and the section_details( this will be a struct with the the inner file path and the edit)
// will return a a vector of the file: edit
pub fn find_section_and_edits(
    program: &str,
    addon: &str,
    sections_details: &Vec<SectionDetail>,
) -> Result<Vec<(String, String, String, bool)>, Box<dyn Error>> {
    let mut edits = Vec::new();

    // Loop through each section detail
    for section in sections_details {
        if section.name == program {
            for subsection in &section.subsections {
                if subsection.name == addon {
                    if subsection.count > 0 {
                        edits.push((
                            subsection.file.clone(),
                            subsection.edit.clone(),
                            subsection.loc.clone(),
                            subsection.after.clone(),
                        ));
                    } else {
                        return Err(Box::new(std::io::Error::new(
                            std::io::ErrorKind::NotFound,
                            format!("No edits required for add-on: {}", addon),
                        )));
                    }
                }
            }
            for edit in edits.iter() {
                println!("edit found: {:?}", edit);
            }
            if edits.is_empty() {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("Add-on not found: {}", addon),
                )));
            }

            return Ok(edits);
        }
    }

    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        format!("Program not found: {}", program),
    )))
}

//return the sections and subsections
pub fn print_sections(
    sections_details: &[SectionDetail],
    req_type: &str,
    needed_section: Option<&str>,
    print: bool,
) -> Result<Vec<String>, Box<dyn Error>> {
    let mut message = Vec::new();
    match req_type {
        "section" => {
            for section in sections_details {
                if print {
                    println!("{}: ", section.name);
                }
                message.push(section.name.clone());
            }
            Ok(message)
        }
        "subsection" => {
            if let Some(needed) = needed_section {
                let mut prev_subsection_name: Option<String> = None;
                for section in sections_details {
                    if &section.name == needed {
                        for subsection in &section.subsections {
                            if prev_subsection_name.as_ref() != Some(&subsection.name) {
                                if print {
                                    println!("- {} ({})", subsection.name, subsection.count);
                                }
                                message.push(subsection.name.clone());
                                prev_subsection_name = Some(subsection.name.clone()); // Update the previous subsection name

                            }
                        }
                    }
                }
            }
            Ok(message)
        }
        "all" => {
            for section in sections_details {
                if print {
                    println!("{}: ", section.name); // Ensure each section name is printed
                }
                message.push(section.name.clone());

                let mut prev_subsection_name: Option<String> = None;

                for subsection in &section.subsections {
                    if prev_subsection_name.as_ref() != Some(&subsection.name) {
                        if print {
                            println!("- {} ({})", subsection.name, subsection.count);
                        }
                        message.push(format!("- {} ({})", subsection.name, subsection.count));
                        prev_subsection_name = Some(subsection.name.clone()); // Update the previous subsection name
                    }
                }
                if print {
                    println!(); // End the line after listing subsections
                }
            }
            Ok(message)
        }
        _ => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Type not found: {}", req_type),
            )));
        }
    }
}
