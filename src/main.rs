use std::{fs::{File}, process::exit};

use xml::EventReader;

fn main() {
    let file_path = "docs.gl/gl4/glClear.xhtml";
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("ERROR: could not read file {file_path}: {err}");
            exit(1);
        }
    };

   let er = EventReader::new(file);

   for event in er.into_iter() {
    println!("{event:?}")
   }


}
