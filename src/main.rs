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

    fn collect_words(&mut self) -> Option<&'a[char]>  {
        let mut n = 0;
        while n < self.content.len() && self.content[n].is_alphanumeric() {
            n += 1;
        }
       let token =  &self.content[0..n];
       self.content = &self.content[n..];
       return Some(token);
    }

    fn collect_numbers(&mut self) -> Option<&'a[char]> {
        let mut n = 0;
        while n < self.content.len() && self.content[n].is_numeric() {
            n += 1;
        }
        let token =  &self.content[0..n];
        self.content = &self.content[n..];
        return Some(token);
    }

    fn next_token(&mut self) -> Option<&'a[char]> {
        self.trim_left();
        if self.content.len() == 0 {
            return None;
        }
        if self.content[0].is_numeric() {
            return self.collect_numbers()
        }
        if self.content[0].is_alphabetic() {
            return self.collect_words()
        }
        let token = &self.content[0..1];
        self.content = &self.content[1..];
        return Some(token);
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

    let content = read_entire_xml_file("docs.gl/gl4/glVertexAttribDivisor.xhtml")?
    .chars()
    .collect::<Vec<_>>();

    for token in Lexer::new(&content) {
        println!("{token}", token = token.iter().collect::<String>());
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
        content.push_str(" ");
    }
}
Ok(content)
}
// return hasmap of each word in the document mapped to its frequency in the document
fn index_document(doc_content: &str) -> HashMap<String, usize> {
    todo!("not implemented")
}
