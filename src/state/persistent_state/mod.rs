use horae::TimeZone;
use nabu::{Object, XffValue};


pub struct PersistentState {
    pub timezone: TimeZone,
    pub file_path: String,
}

impl PersistentState {
    pub fn new(timezone: TimeZone, file_path: String) -> PersistentState {
        PersistentState {
            timezone,
            file_path,
        }
    }

    pub fn make_persistent(&self) {
        let serialized = self.serialize();
        let save_state = nabu::serde::write(&self.file_path, serialized);
        // TODO: error handling
    }

    fn serialize(&self) -> XffValue {
        let mut obj = Object::new();
        obj.insert("timezone", self.timezone.to_string());
        obj.insert("file_path", self.file_path.clone());

        XffValue::from(obj)
    }
}

impl Default for PersistentState {
    fn default() -> PersistentState {
        PersistentState {
            timezone: TimeZone::CoordinatedUniversalTime,
            file_path: "todo.txt".to_string(),
        }
    }
}
