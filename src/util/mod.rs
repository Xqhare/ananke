use horae::{TimeZone, Utc};

pub struct NewTask {
    pub prio: String,
    pub inception_date: String,
    pub text: String,
}

impl NewTask {
    pub fn new(timezone: TimeZone) -> Self {
        let date = {
            let mut now = Utc::now();
            now.with_timezone(timezone);
            now.date().to_string()
        };
        NewTask { 
            prio: String::default(),
            inception_date: date,
            text: String::default(),
        }
    }
}
