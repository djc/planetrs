use std::fs::File;
use std::io::Write;

use atom_syndication;

use entry::Entry;

pub fn export(entries: &Vec<Entry>) {
    let mut atom_entries = Vec::<atom_syndication::Entry>::new();
    for entry in entries.iter().take(20) {
        let main_link = atom_syndication::Link {
            href: entry.link.clone(),
            ..Default::default()
        };
        let temp_entry = atom_syndication::Entry {
            id: entry.uid.clone(),
            title: entry.title.clone(),
            updated: entry.date.to_rfc3339(),
            links: vec![main_link],
            ..Default::default()
        };
        atom_entries.push(temp_entry);
    }

    let main_link = atom_syndication::Link {
        href: "www.planet-rust.com".to_owned(),
        ..Default::default()
    };
    let last_update = entries[0].date;

    let atom_feed = atom_syndication::Feed {
        id: String::from("www.planet-rust.com"),
        title: String::from("Planet Rust"),
        links: vec![main_link],
        updated: last_update.to_rfc3339(),
        entries: atom_entries,
        ..Default::default()
    };

    let mut f = File::create("html/atom.xml").expect("Cant create atom file");
    f.write_all(&atom_feed.to_string().into_bytes())
        .expect("Cant write atom file");
}
