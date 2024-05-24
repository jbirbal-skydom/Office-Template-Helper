// src/modify.rs
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::{Reader, Writer};
use std::io::{self, Cursor};

/// Modifies XML content according to specific rules.
pub fn modify_xml(xml_content: &String, data: &String, loc: &String, before: &bool) -> io::Result<String> {
    let mut reader = Reader::from_str(&xml_content);
    reader.trim_text(true);
    let mut in_element2 = false; // Flag to detect when inside `element2`

    let mut writer = Writer::new(Cursor::new(Vec::new()));

    loop {
        match reader.read_event() {
            // Ok(Event::Start(e)) if e.name().as_ref() == b"mod" => {

            //     // crates a new element ... alternatively we could reuse `e` by calling
            //     // `e.into_owned()`
            //     let mut elem = e.clone().into_owned();

            //     // collect existing attributes
            //     elem.extend_attributes(e.attributes().map(|attr| attr.unwrap()));

            //     // copy existing attributes, adds a new my-key="some value" attribute
            //     // elem.push_attribute(("modified", "true"));

            //     // writes the event to the writer
            //     assert!(writer.write_event(Event::Start(elem)).is_ok());
            // },
            Ok(Event::Start(ref e)) if e.name().as_ref() == b"element2" => {
                // Detecting start of `element2`
                in_element2 = true;
                writer.write_event(Event::Start(e.to_owned())).unwrap();
            }
            Ok(Event::End(ref e)) if e.name().as_ref() == b"element2" => {
                // End of `element2`
                in_element2 = false;
                writer.write_event(Event::End(e.to_owned())).unwrap();
            }
            Ok(Event::Text(_e)) if in_element2 => {
                // Change the text content of `element2`
                let text = BytesText::new("New Value 2");
                writer.write_event(Event::Text(text)).unwrap();
            }

            Ok(Event::End(e)) if e.name().as_ref() == b"element3" => {
                writer
                    .write_event(Event::End(BytesEnd::new("element3")))
                    .unwrap();
                // add element4 after element3
                // After the end tag of element3, insert new element4
                let elem_start = BytesStart::new("element4");
                writer.write_event(Event::Start(elem_start)).unwrap();

                let elem_text = BytesText::new("Value 4");
                writer.write_event(Event::Text(elem_text)).unwrap();

                let elem_end = BytesEnd::new("element4");
                writer.write_event(Event::End(elem_end)).unwrap();
            }
            Ok(Event::Eof) => break,
            // we can either move or borrow the event to write, depending on your use-case
            Ok(e) => assert!(writer.write_event(e).is_ok()),
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
        }
    }

    // Collect results into a string
    let result = writer.into_inner().into_inner();
    Ok(String::from_utf8(result).expect("Failed to parse bytes to string"))
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
