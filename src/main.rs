use std::{fs::{self, File}, io, path::Path};

use xml::{reader::XmlEvent, EventReader};

fn main() -> io::Result<()> {
    let dir_path = "docs.gl/gl4";
    let dir = fs::read_dir(dir_path)?;

    for file in dir  {
       let  file_path = file?.path();
        let content = read_entire_xml_file(&file_path)?;
        println!("{file_path:?} => {size} ", size = content.len());
    }
    Ok(())
}

fn read_entire_xml_file<P: AsRef<Path>>(file_path: P) -> io::Result<String> {
    let mut content = String::new();
    let file = File::open(file_path)?;
   let er = EventReader::new(file);
   for event in er.into_iter() {
    let event = event.expect("TODO");
    if let XmlEvent::Characters(text ) = event {
        content.push_str(&text);
    }
}
Ok(content)
}
