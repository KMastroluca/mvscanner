use crate::database::db::DB;
use serde::{Deserialize, Serialize};

/*
*   "name": "John Doe",
    "doc": 123345,
    "rfid_tag": 123455623562354,
    "housing": {
      "unit": 3,
      "pod_room": "A-1B"
    },
    "location": null,
    "timestamps": [],
    "current_timestamp": null
*/

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Resident {
    pub rfid_tag: String, // rfid_tag: 123455623562354
    pub name: String,     // name: Last, First
    pub doc: String,      // doc: 123345
    pub room: String,
}

impl Resident {
    pub fn new(rfid_tag: String, name: String, doc: String, room: String) -> Self {
        Self {
            rfid_tag,
            name,
            doc,
            room,
        }
    }

    pub fn index(db: &DB) -> Vec<Self> {
        db.index_residents().unwrap_or_default()
    }

    pub fn store(resident: &Resident, db: &DB) -> Result<(), rusqlite::Error> {
        db.store_resident(resident)
    }

    pub fn destroy(rfid: &str, db: &DB) -> Result<(), rusqlite::Error> {
        db.delete_resident(rfid)
    }

    pub fn update(resident: &Resident, db: &DB) -> Result<(), rusqlite::Error> {
        db.update_resident(resident)
    }

    pub fn show(rfid: &str, db: &DB) -> Option<Self> {
        if let Ok(res) = db.show_resident(rfid) {
            Some(res)
        } else {
            None
        }
    }

    pub fn delete(rfid: &str, db: &DB) -> Result<(), rusqlite::Error> {
        db.delete_resident(rfid)
    }

    pub fn get_test_residents_from_file() -> Vec<Resident> {
        let file = std::fs::read_to_string("seed_data/residents.json").unwrap();
        serde_json::from_str::<Vec<Resident>>(&file).unwrap()
    }
}
