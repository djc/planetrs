#[macro_use]
extern crate serde_derive;
extern crate curl;
extern crate chrono;
extern crate atom_syndication;
extern crate rss;
extern crate serde_yaml;
extern crate serde_json;
extern crate tera;
extern crate uuid;
extern crate clap;

mod entry;
mod reader;
mod catcher;
mod renderer;
mod storer;
mod atom_exporter;

fn main() {
    let matches = clap::App::new("planetrs")
        .version("0.1.0")
        .author("Adau/Vagdish <adrien.aubourg@gmail.com>")
        .about("Generation of static html files from a list of atom/rss feeds (planet)")
        .arg(clap::Arg::with_name("quiet")
                 .help("Don't log status updates to stdout")
                 .short("q")
                 .long("quiet"))
        .arg(clap::Arg::with_name("config")
                 .help("Configuration file path")
                 .required(true)
                 .takes_value(true))
        .get_matches();

    let quiet = matches.is_present("quiet");
    let config_filepath = matches
        .value_of("config")
        .expect("No config file entered");
    let mut data = reader::read_configfile(config_filepath);
    let mut entries = catcher::get_entries(&data.feeds, quiet);
    storer::merge_entries(&mut entries, "entries.json");
    entries.truncate(12);
    data.entries = entries;
    atom_exporter::export(&data.entries);

    renderer::render(&data);
}
