use horae::TimeZone;
use nabu::{Object, XffValue};

use crate::error::AnankeError;

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

    pub fn read_persistent<P: AsRef<std::path::Path>>(path: P) -> Result<PersistentState, AnankeError> {
        let deserialized = nabu::serde::read(path);
        if let Err(e) = deserialized {
            Err(AnankeError::new(
                "Reading file error",
                "Error reading state from disk",
                Some(&e.to_string()),
            ))
        } else {
            Self::deserialize(deserialized.unwrap())
        }
    }

    fn deserialize(xff_value: XffValue) -> Result<PersistentState, AnankeError> {
        if let XffValue::Object(obj) = xff_value {
            let timezone = TimeZone::from(obj.get("timezone").unwrap().into_string().unwrap());
            Ok(PersistentState {
                timezone,
                file_path: obj.get("file_path").unwrap().into_string().unwrap(),
            })
        } else {
            Err(AnankeError::new(
                "Reading file error",
                "Error reading state from disk",
                Some("XffValue is not an object"),
            ))
        }
    }

    pub fn make_persistent(&self) -> Result<(), AnankeError> {
        let serialized = self.serialize();
        let save_state = nabu::serde::write(&self.file_path, serialized);
        if let Err(e) = save_state {
            Err(AnankeError::new(
                "Writing file error",
                "Error writing state to disk",
                Some(&e.to_string()),
            ))
        } else {
            Ok(())
        }
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
