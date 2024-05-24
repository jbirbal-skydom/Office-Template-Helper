// src/file_handler.rs
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use zip::write::SimpleFileOptions;
use zip::{ZipArchive, ZipWriter};

// check file validity and check the file extension and return the file extension and path of the file
pub fn check_file(file_path: &str) -> (String, String) {
    let path = Path::new(file_path);
    let ext = path.extension().unwrap().to_str().unwrap().to_owned();
    let path = path.to_str().unwrap().to_owned();
    (ext, path)
}

/// Changes the extension of a file to the original extension specified.
pub fn change_extension_to_original(file_path: &str, original_ext: &str) -> String {
    let path = Path::new(file_path);
    let new_path = path.with_extension(original_ext);
    fs::rename(&path, &new_path).expect("Failed to rename file");
    new_path.to_str().unwrap().to_owned()
}

/// Opens a .zip file and extracts the file paths and their contents,
/// optionally including the ZIP file's own path in the results.
pub fn open_zip(file_path: &str, include_zip_path: bool) -> io::Result<Vec<(String, String)>> {
    let file = File::open(file_path)?;
    let mut archive = ZipArchive::new(file)?;
    let mut entries = Vec::new();
    let file_path_buf = Path::new(file_path).canonicalize()?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let full_path = if include_zip_path {
            // If including the ZIP path, prepend it to the file's relative path within the ZIP
            let mut path_buf = file_path_buf.clone();
            path_buf.push(file.name()); // Properly handle paths on all platforms
            path_buf.to_string_lossy().into_owned()
            // format!("{}/{}", file_path, file.name())
        } else {
            // Otherwise, just use the relative path within the ZIP
            file.name().replace("\\", "/") // Ensure uniform separators
        };

        entries.push((full_path, contents));
    }

    Ok(entries)
}

/// Writes content to a ZIP file and compresses it.
pub fn write_content_to_zip(zip_path: &str, inner_path: &str, content: &str) -> io::Result<()> {
    let path = Path::new(zip_path);
    let file = File::create(&path)?;
    let mut zip = ZipWriter::new(file);

    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);

    println!("writing to zip file:{:?} {}", path, inner_path);
    zip.start_file(inner_path, options)?;
    zip.write_all(content.as_bytes())?;
    zip.finish()?;
    Ok(())
}

//copy the original zip and make a new zip file called `modified` which will be used for processing
// the old zip will not be modified and the new zip will ne identical to the old zip
// the only argument will be the file locatio and the return will be the new file location
pub fn copy_zip(file_path: &str) -> io::Result<String> {
    let path = Path::new(file_path).canonicalize()?;

    let new_path = path.with_file_name("modified.zip");
    fs::copy(&path, &new_path)?;
    Ok(new_path.to_str().unwrap().to_owned())
}
