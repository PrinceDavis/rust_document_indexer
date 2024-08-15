use std::{collections::HashMap, fs::{self, File}, io, path::{Path, PathBuf}};
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

    fn collect_words(&mut self) -> Option<String>  {
        let mut n = 0;
        while n < self.content.len() && self.content[n].is_alphanumeric() {
            n += 1;
        }
       self.chop(n)
    }

    fn collect_numbers(&mut self) -> Option<String> {
        let mut n = 0;
        while n < self.content.len() && self.content[n].is_numeric() {
            n += 1;
        }
        self.chop(n)
    }

    fn chop(&mut self, n: usize) -> Option<String>{
        let token = &self.content[0..n].iter().collect::<String>();
        self.content = &self.content[n..];
        Some(token.to_owned())
    }

    fn next_token(&mut self) -> Option<String> {
        self.trim_left();
        if self.content.len() == 0 {
            return None;
        }
        if self.content[0].is_numeric() {
            return Some(self.collect_numbers().iter().map(|x| x.to_ascii_uppercase()).collect());
        }
        if self.content[0].is_alphabetic() {
            return Some(self.collect_words().iter().map(|x| x.to_ascii_uppercase()).collect());
        }
        self.chop(1)
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

type TermFreq = HashMap::<String, usize>;
type TermFreqIndex = HashMap<PathBuf, TermFreq>;

fn main() -> io::Result<()> {
    let index_path = "index.json";
   let index_file =  File::open(index_path)?;
   println!("Reading {index_path} index file...");
   let tf_index: TermFreqIndex = serde_json::from_reader(index_file).expect("reading file to TermFreqIndex successful");
    search("bind, to buffer".to_string(), tf_index);
   Ok(())
}

fn index_folder() -> io::Result<()> {
    let dir_path = "docs.gl/gl4";
    let dir = fs::read_dir(dir_path)?;
    let mut tf_index = TermFreqIndex::new();

    for file in dir  {
       let  file_path = file?.path();
        let content = read_entire_xml_file(&file_path)?
        .chars()
        .collect::<Vec<_>>();

        println!("Indexing {file_path:?}");
        let mut tf = TermFreq::new();
        for token in Lexer::new(&content) {
            if let Some(freq) = tf.get_mut(&token) {
                *freq +=1;
            }else {
                tf.insert(token, 1);
            }
        }

        let mut stats = tf.iter().collect::<Vec<_>>();
        stats.sort_by_key(|(_, f)| *f);
        stats.reverse();
        tf_index.insert(file_path, tf);
    }

    let index_path = "index.json";
    println!("Saving {index_path}...");
    // let j = serde_json::to_string(&tf_index)?;
    let index_file = File::create(index_path)?;
    serde_json::to_writer(index_file, &tf_index).expect("serde works fine");
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

fn search(keywords: String, tf_index: TermFreqIndex) {
    let mut results = Vec::<(PathBuf, f32)>::new();
    for (path, tf_table) in &tf_index {
        let mut rank = 0f32;
        for token in Lexer::new(&keywords.chars().collect::<Vec<_>>()) {
            rank += tf(&token, &tf_table) * idf(&token, &tf_index);
        }
        results.push((path.clone(), rank))
    }
    results.sort_by(|(_, rank1), (_, rank2)| rank1.partial_cmp(rank2).unwrap());
    results.reverse();
    for (path, rank) in results {
        println!("{path} => {rank}", path = path.display());
    }
}

fn tf(term: &str, doc: &TermFreq) -> f32 {
    let numerator = doc.get(term).cloned().unwrap_or(0) as f32;
    let denomitor = doc.iter().map(|(_, f)| *f).sum::<usize>() as f32;
    numerator / denomitor
 }

 fn idf(t :&str, d: &TermFreqIndex) -> f32 {
    let n = d.len() as f32;
    let m =  d.values().filter(| tf| tf.contains_key(t)).count().max(1) as f32;
     (n /  m).log10()
 }

