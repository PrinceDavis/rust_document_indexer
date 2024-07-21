use std::{fs::File, io, process::exit};

use xml::{reader::XmlEvent, EventReader};

fn main() {
    let file_path = "docs.gl/gl4/glClear.xhtml";
    let content = read_entire_xml_file(file_path).unwrap_or_else(|err| {
        eprintln!("ERROR: could not read the xml {file_path} : {err}");
        exit(1);
    });
    println!("{content}")
}

fn read_entire_xml_file(file_path: &str) -> io::Result<String> {
    let mut content = String::new();
    let file = File::open(file_path)?;

   let er = EventReader::new(file);

   for event in er.into_iter() {
    let event = event.expect("TODO");
    if let XmlEvent::Characters(text ) = event {
        content.push_str(&text);
        print!("{text}");
    }
}
Ok(content)
}
