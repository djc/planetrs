use std::fs::File;
use std::io::prelude::*;

use tera::{Tera, Context};

use entry::Entry;
use entry::FeedInfo;

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub title: String,
    pub subtitle: String,
    pub entries_per_page: u32,
    pub entries_in_atom: u32,

    pub feeds: Vec<FeedInfo>,
    pub entries: Vec<Entry>,
}

impl Data {
    pub fn new() -> Data {
        Data {title: String::new(),
              subtitle: String::new(),
              entries_per_page: 0u32,
              entries_in_atom: 0u32,
              feeds: Vec::new(),
              entries: Vec::new()}
    }
}

pub fn render(data: &Data) {
    // let mut main_path = data.template_folder.clone();
    // main_path.set_file_name("main.html");

    let mut tera = Tera::new("./templates/**/*.html").expect("Cant compile html");
    tera.autoescape_on(vec![]);
    let mut context = Context::new();
    context.add("data", data);
    let output = tera.render("templates/main.html", context).expect("Tera couldnt render output");
    let mut f = File::create("./html/index.html").expect("Cant create file for html output");
    let _ = f.write_all(output.as_bytes());
}
