use std::string::String;
use std::time::Duration;
use std::thread;
use std::sync::{Arc, Mutex};

use curl::easy::Easy;
use chrono;
use rss;
use atom_syndication;

use entry::FeedInfo;
use entry::Entry;

pub fn get_entries(feeds: &[FeedInfo], quiet: bool) -> Vec<Entry> {
    let inner_fi = feeds.to_owned();
    let mut th_entries = Arc::new(Mutex::new(Vec::<Entry>::new()));

    let mut handles = Vec::new();
    for fi in inner_fi {
        let th_entries = Arc::clone(&th_entries);
        handles.push(thread::spawn(move || {
            let mut dst = Vec::new();
            let mut handle = Easy::new();
            handle
                .timeout(Duration::new(10, 0))
                .expect("Cant set timeout");
            handle.url(&fi.feedurl).expect("Cant set url");
            handle.get(true).expect("Cant set Get");
            {
                let mut transfer = handle.transfer();
                transfer
                    .write_function(|data| {
                                        dst.extend_from_slice(data);
                                        Ok(data.len())
                                    })
                    .expect("Cant set write_fn");
                match transfer.perform() {
                    Err(e) => println!("Perform() failed ({}): {}", fi.id, e),
                    Ok(_) => if !quiet {
                        println!("Successful download of {}", fi.id)
                    },
                }
            }

            let buf = String::from_utf8(dst).expect("Cant convert dst to buf");
            if let Ok(f) = buf.parse::<rss::Channel>() {
                rss_to_entries(&f, &fi, &th_entries)
            } else if let Ok(f) = buf.parse::<atom_syndication::Feed>() {
                atom_to_entries(&f, &fi, &th_entries)
            } else {
                println!("Cant parse feed: {}", fi.id);
                println!("{:?}", buf.parse::<rss::Channel>());
                println!("{:?}", buf.parse::<atom_syndication::Feed>());
            }
        }))
    }

    for h in handles {
        let _ = h.join();
    }

    Arc::get_mut(&mut th_entries)
        .expect("getmut arc failed")
        .get_mut()
        .expect("getmut mutex failed")
        .clone()
}

fn rss_to_entries(f: &rss::Channel, info: &FeedInfo, v: &Arc<Mutex<Vec<Entry>>>) {
    for item in &f.items {
        let mut entry = Entry::new();
        entry.info = (*info).clone();
        entry.title = item.clone().title.expect("rss title failed");
        entry.link = item.clone().link.expect("rss link failed");
        let temp_resume = item.clone().description.expect("rss content failed");
        entry.resume = select_first_paragraph(&temp_resume);
        entry.date = chrono::DateTime::parse_from_rfc2822(item.clone()
                                                              .pub_date
                                                              .expect("rss date failed")
                                                              .replace("UTC", "+0000")
                                                              .as_ref())
                .expect("parse date failed")
                .with_timezone(&chrono::Utc);
        entry.generate_human_date();
        v.lock().expect("v lock failed").push(entry);
    }
}

fn atom_to_entries(f: &atom_syndication::Feed, info: &FeedInfo, v: &Arc<Mutex<Vec<Entry>>>) {
    for item in f.entries() {
        let mut entry = Entry::new();
        entry.info = (*info).clone();
        entry.title = item.clone().title().to_string();
        entry.link = item.clone().links()[0].clone().href().to_string();
        if let Some(content) = item.content() {
            match content.content_type() {
                Some("text") | Some("html") | Some("xhtml") => {
                    entry.resume = select_first_paragraph(content.value().unwrap());
                },
                Some(_) | None => {},
            }
        }
        let temp = item.updated();
        entry.date = chrono::DateTime::parse_from_rfc3339(temp.as_ref())
            .expect("rss date failed")
            .with_timezone(&chrono::Utc);
        entry.generate_human_date();
        v.lock().expect("v lock failed").push(entry);
    }
}

fn select_first_paragraph(txt: &str) -> String {
    let temp_str = txt.replace("&lt;", "<").replace("&gt;", ">");
    if temp_str.split("<p>").nth(1).is_some() {
        let temp_str = temp_str.split("<p>").nth(1).expect("Bad <p> split");
        let temp_str = temp_str.split("</p>").nth(0).expect("Bad </p> split");
        temp_str.to_string()
    } else {
        temp_str.to_string()
    }
}
