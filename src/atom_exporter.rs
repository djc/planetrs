use std::fs::File;
use std::io::Write;

use atom_syndication;

use entry::Entry;

pub fn export(entries: &[Entry]) {
    let mut atom_entries = Vec::<atom_syndication::Entry>::new();
    for entry in entries.iter().take(20) {
        let mut main_link = atom_syndication::Link::default();
        main_link.set_href(entry.link.as_ref());
        let mut temp_entry = atom_syndication::Entry::default();
        temp_entry.set_id(entry.uid.as_ref());
        temp_entry.set_title(entry.title.as_ref());
        temp_entry.set_updated(entry.date.to_rfc3339());
        temp_entry.set_links(vec![main_link]);
        atom_entries.push(temp_entry);
    }

    let mut main_link = atom_syndication::Link::default();
    main_link.set_href("http://www.planet-rust.com/");
    let last_update = entries[0].date;

    let mut atom_feed = atom_syndication::Feed::default();
    atom_feed.set_id("www.planet-rust.com");
    atom_feed.set_title("Planet Rust");
    atom_feed.set_links(vec![main_link]);
    atom_feed.set_updated(last_update.to_rfc3339());
    atom_feed.set_entries(atom_entries);

    let mut f = File::create("html/atom.xml").expect("Cant create atom file");
    f.write_all(&atom_feed.to_string().into_bytes())
        .expect("Cant write atom file");
}
