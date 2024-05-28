// src/file_handler.rs
use std::fs::{self, File};
use std::io::Read;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use zip::write::SimpleFileOptions;
use zip::{ZipArchive, ZipWriter};
use rand::{distributions::Alphanumeric, Rng};

fn generate_random_path_safe_string(len: usize) -> String {
    let rng = rand::thread_rng();
    rng.sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

// check file validity and check the file extension and return the file extension and path of the file
pub fn check_file(file_path: &str) -> (String, String) {
    let path = Path::new(file_path);
    let ext = path.extension().unwrap().to_str().unwrap().to_owned();
    let path = path.to_str().unwrap().to_owned();
    (ext, path)
}

// Changes the extension of a file to the original extension specified.
pub fn change_extension_to_original(file_path: &str, original_ext: &str) -> String {
    let path = Path::new(file_path);
    let new_path = path.with_extension(original_ext);
    fs::rename(&path, &new_path).expect("Failed to rename file");
    new_path.to_str().unwrap().to_owned()
}

pub fn change_extension_to_modified(new_file_path: &str, original_file_path: &str,original_ext: &str) -> String {
    let path = Path::new(new_file_path);
    let mut new_path = PathBuf::from(path);
    let random_string = generate_random_path_safe_string(4);
    let original_file_name = Path::new(original_file_path)
        .file_stem()  // This gets the filename without the extension
        .and_then(|os_str| os_str.to_str())  // Convert OsStr to &str
        .unwrap_or_default();  // Provide a default empty string if any operation fails    println!(   "original file name: {}", original_file_name);
    let filename = format!("{}-{}(modified)", original_file_name, random_string);
    println!("filename: {}", filename);

    // Change the file name to "modified" while preserving the directory structure
    if let Some(parent) = path.parent() {
        new_path.set_file_name(filename);
        new_path.set_extension(original_ext);
        new_path = parent.join(new_path);
        println!("new path1: {:?}", new_path);
    } else {
        // If there's no parent (e.g., relative path without directories), just modify the name and extension
        new_path.set_file_name(filename);
        new_path.set_extension(original_ext);
        println!("new path2: {:?}", new_path)
    }

    // Perform the file renaming
    fs::rename(&path, &new_path).expect("Failed to rename file");

    // Return the new path as a String
    new_path.to_str().unwrap().to_owned()
}

/// Opens a .zip file and extracts the file paths and their contents,
/// optionally including the ZIP file's own path in the results.
pub fn open_zip(file_path: &str, include_zip_path: bool) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let mut archive = ZipArchive::new(file)?;
    let mut paths = Vec::new();
    let file_path_buf = Path::new(file_path).canonicalize()?;

    for i in 0..archive.len() {
        let file = archive.by_index(i)?;
        let full_path = if include_zip_path {
            let mut path_buf = file_path_buf.clone();
            path_buf.push(file.name()); // Properly handle paths on all platforms
            path_buf.to_string_lossy().into_owned()
        } else {
            file.name().replace("\\", "/") // Ensure uniform separators
        };

        paths.push(full_path);
    }

    Ok(paths)
}

/// Writes content to a ZIP file and compresses it.
/// Modifies content of a specific file within a zip archive, keeping other files intact.
pub fn write_content_to_zip(source: &String, target_file: &str, modify: &String) -> io::Result<()> {
    let temp_path = format!("{}.tmp", source);
    let destination = Path::new(&temp_path);
    let reader = File::open(source)?;
    let mut zip = ZipArchive::new(reader)?;

    let writer = File::create(destination)?;
    let mut zip_writer = ZipWriter::new(writer);

    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);

        if file.name() == target_file {
            let mut contents = Vec::new();
            file.read_to_end(&mut contents)?;
            let modified_contents = modify.as_bytes();
            zip_writer.start_file(file.name(), options)?;
            zip_writer.write_all(&modified_contents)?;
        } else {
            let mut contents = Vec::new();
            file.read_to_end(&mut contents)?;
            zip_writer.start_file(file.name(), options)?;
            zip_writer.write_all(&contents)?;
        }
    }

    zip_writer.finish()?;

    // Replace the original file with the new zip
    fs::remove_file(source)?;  // Delete the original file
    fs::rename(destination, source)?;  // Rename the temporary file to the original file name


    Ok(())
}

//copy the original zip and make a new zip file called `modified` which will be used for processing
// the old zip will not be modified and the new zip will ne identical to the old zip
// the only argument will be the file location and the return will be the new file location
pub fn copy_zip(file_path: &str) -> io::Result<String> {
    let path = Path::new(file_path).canonicalize()?;
    let random_string = generate_random_path_safe_string(10);
    let filename = random_string + ".zip";
    let new_path = path.with_file_name(filename);
    fs::copy(&path, &new_path)?;
    Ok(new_path.to_str().unwrap().to_owned())
}

pub fn read_zip_file_content(zip_file_path: &str, internal_file_path: &str) -> io::Result<Vec<u8>> {
    let file = File::open(zip_file_path)?;
    let mut archive = ZipArchive::new(file)?;

    // print the file being read
    println!("reading file: {} - {}", zip_file_path, internal_file_path);

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;

        if file.name() == internal_file_path {
            let mut contents = Vec::new();
            file.read_to_end(&mut contents)?;
            return Ok(contents);
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "File not found in the ZIP archive",
    ))
}


