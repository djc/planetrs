use std::fs::File;
use std::path::Path;

use serde_yaml;

use entry::FeedInfo;
use renderer::Data;

pub fn read_configfile<P: AsRef<Path>>(path: P) -> Data {
    let mut data = Data::new();
    let filestream = File::open(path).expect("Couldn't read feeds file");
    let yaml_map: serde_yaml::Mapping = serde_yaml::from_reader(filestream).expect("Couldn't parse feeds list");

    data.title = yaml_map.get(&serde_yaml::Value::String("Title".to_string())).unwrap().as_str().unwrap().to_string();
    data.subtitle = yaml_map.get(&serde_yaml::Value::String("Subtitle".to_string())).unwrap().as_str().unwrap().to_string();

    data.entries_per_page = yaml_map.get(&serde_yaml::Value::String("Entries_per_page".to_string())).unwrap().as_i64().unwrap() as u32;
    data.entries_in_atom = yaml_map.get(&serde_yaml::Value::String("Entries_in_atom".to_string())).unwrap().as_i64().unwrap() as u32;

    let feeds_seq = yaml_map.get(&serde_yaml::Value::String("Feeds".to_string())).unwrap().as_sequence().unwrap();
    for feed in feeds_seq {
        data.feeds.push(parse_feed(feed));
    }

    data
}

fn parse_feed(yml_feed: &serde_yaml::Value) -> FeedInfo {
    let mut fi = FeedInfo::new();
    let feed_map = yml_feed.as_mapping().expect("Couldnt parse feed as map");

    let id_key = &serde_yaml::Value::String("name".to_string());
    let name_key = &serde_yaml::Value::String("name".to_string());
    let url_key = &serde_yaml::Value::String("feedurl".to_string());
    let home_key = &serde_yaml::Value::String("homepage".to_string());

    fi.id = feed_map.get(id_key).expect("value name").as_str().expect("name str").to_string();
    fi.name = feed_map.get(name_key).expect("value name").as_str().expect("name str").to_string();
    fi.feedurl = feed_map.get(url_key).expect("value feedurl").as_str().expect("url str").to_string();
    fi.homepage = feed_map.get(home_key).expect("value homepage").as_str().expect("home str").to_string();

    fi
}
