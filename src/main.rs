use std::{collections::HashMap, fs::{self, File}, io, path::Path};
use xml::{reader::XmlEvent, EventReader};

#[derive(Debug)]
struct Lexer<'a> {
    content: &'a [char],
}

impl <'a> Lexer<'a> {
    fn new(content: &'a [char]) -> Self {
        Self{content}
    }

    fn trim_left(&mut self) {
        while self.content.len() > 0 && self.content[0].is_whitespace() {
            self.content = &self.content[1..];
        }
    }

    fn next_token(&mut self) -> Option<&'a[char]> {
        self.trim_left();
        if self.content.len() == 0 {
            return None;
        }
        todo!()
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = &'a[char];

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

fn main() -> io::Result<()> {
    let dir_path = "docs.gl/gl4";
    let dir = fs::read_dir(dir_path)?;

    let content = read_entire_xml_file("docs.gl/gl4/glVertextAttribDivisor.xhtml")?
    .chars()
    .collect::<Vec<_>>();

    for token in Lexer::new(&content) {
        println!("{token:?}");
    }

    // let all_documents = HashMap::<Path, HashMap<String, usize>>::new();

    // for file in dir  {
    //    let  file_path = file?.path();
    //     let content = read_entire_xml_file(&file_path)?;
    //     println!("{file_path:?} => {size} ", size = content.len());
    // }
    Ok(())
}

// read the content of an xmpl file
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
// return hasmap of each word in the document mapped to its frequency in the document
fn index_document(doc_content: &str) -> HashMap<String, usize> {
    todo!("not implemented")
}
