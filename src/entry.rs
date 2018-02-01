use chrono;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entry {
    pub info: FeedInfo,
    pub title: String,
    pub date: chrono::DateTime<chrono::Utc>,
    pub hdate: String,
    pub link: String,
    pub resume: String,
}

impl Entry {
    pub fn new() -> Entry {
        Entry {
            info: FeedInfo::new(),
            title: String::new(),
            date: chrono::Utc::now(),
            hdate: String::new(),
            link: String::new(),
            resume: String::new(),
        }
    }

    pub fn generate_human_date(&mut self) {
        self.hdate = self.date.format("%B %d, %Y").to_string();
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FeedInfo {
    pub id: String,
    pub name: String,
    pub feedurl: String,
    pub homepage: String,
}

impl FeedInfo {
    pub fn new() -> FeedInfo {
        FeedInfo {
            id: String::new(),
            name: String::new(),
            feedurl: String::new(),
            homepage: String::new(),
        }
    }
}
