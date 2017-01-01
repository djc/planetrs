use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use tera::{Tera, Context};

use entry::Entry;
use entry::FeedInfo;

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub title: String,
    pub subtitle: String,
    pub storage_filepath: String,
    pub template_folder: String,
    pub output_folder: String,
    pub entries_per_page: u32,
    pub entries_in_atom: u32,

    pub feeds: Vec<FeedInfo>,
    pub entries: Vec<Entry>,
}

impl Data {
    pub fn new() -> Data {
        Data {title: String::new(),
              subtitle: String::new(),
              storage_filepath: String::new(),
              template_folder: String::new(),
              output_folder: String::new(),
              entries_per_page: 0u32,
              entries_in_atom: 0u32,
              feeds: Vec::new(),
              entries: Vec::new()}
    }
}

pub fn render<P: AsRef<Path>>(data: &Data, template_name: &str, outputfile: P) {
    let mut tera = Tera::new("./templates/**/*.html").expect("Cant compile html");
    tera.autoescape_on(vec![]);
    let mut context = Context::new();
    context.add("data", data);
    let output = tera.render(template_name, context).expect("Tera couldnt render output");
    let mut f = File::create(outputfile).expect("Cant create file for html output");
    let _ = f.write_all(output.as_bytes());
}
