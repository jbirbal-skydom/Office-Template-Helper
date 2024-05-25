// src/modify.rs
use quick_xml::events:: Event;
use quick_xml::Error as XmlError;
use quick_xml::{Reader, Writer};

use std::io::{self, Cursor, Error as IoError, ErrorKind};

/// Modifies XML content according to specific rules.
pub fn modify_xml(xml_content: &str, data: &str, loc: &str, after: bool) -> io::Result<String> {
    let mut reader = Reader::from_str(xml_content);
    reader.trim_text(true);

    let mut writer = Writer::new(Cursor::new(Vec::new()));

    loop {
        match reader.read_event() {
            Ok(Event::End(ref e)) if e.name().as_ref() == loc.as_bytes() => {
                // Write the original end event first if insertion should be after
                if after {
                    writer
                        .write_event(Event::End(e.to_owned()))
                        .map_err(xml_to_io_error)?;
                }

                // Parse the data as a new XML element and insert
                insert_xml_element(&mut writer, data)?;

                // If after, write the end tag after inserting the new element
                if !after {
                    writer
                        .write_event(Event::End(e.to_owned()))
                        .map_err(xml_to_io_error)?;
                }
            }
            Ok(Event::Eof) => break,
            Ok(e) => writer.write_event(e).map_err(xml_to_io_error)?,
            Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e.to_string())),
        }
    }

    Ok(String::from_utf8(writer.into_inner().into_inner())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?)
}

fn insert_xml_element(writer: &mut Writer<Cursor<Vec<u8>>>, data: &str) -> io::Result<()> {
    // Assuming data is in the form "<elementTag>content</elementTag>"
    let mut tag_reader = Reader::from_str(data);
    tag_reader.trim_text(true);

    // Read the tag and content from the provided data string
    loop {
        match tag_reader.read_event() {
            Ok(Event::Start(ref e)) => writer
                .write_event(Event::Start(e.to_owned()))
                .map_err(xml_to_io_error)?,
            Ok(Event::Text(ref e)) => writer
                .write_event(Event::Text(e.to_owned()))
                .map_err(xml_to_io_error)?,
            Ok(Event::End(ref e)) => {
                writer
                    .write_event(Event::End(e.to_owned()))
                    .map_err(xml_to_io_error)?;
                break; // Exit after the end tag is processed
            }
            Ok(Event::Empty(ref e)) => {
                writer.write_event(Event::Empty(e.to_owned())).map_err(xml_to_io_error)?;
                break; // Exit as this is a self-closing tag
            },
            Ok(Event::Eof) => break, // Handle case where data may not be a complete element
            _ => (),                 // Ignore other events for simplicity
        }
    }

    Ok(())
}

fn xml_to_io_error(e: XmlError) -> IoError {
    IoError::new(ErrorKind::Other, e.to_string())
}
/// Takes XML data as a byte vector and writes it in a prettified format.
pub fn prettify_xml(xml_bytes: &String) -> io::Result<String> {
    let mut reader = Reader::from_reader(xml_bytes.as_bytes());
    reader.trim_text(true);
    let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 4);

    loop {
        match reader.read_event() {
            Ok(Event::Eof) => break, // Exit the loop when end of file is reached
            Ok(event) => {
                // Write the event to the writer
                writer.write_event(&event).unwrap();
            }
            Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData, e.to_string())), // Handle potential errors
        }
    }

    // Convert results back to a String
    let result_bytes = writer.into_inner().into_inner();
    let result_string = String::from_utf8(result_bytes)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

    Ok(result_string)
}
