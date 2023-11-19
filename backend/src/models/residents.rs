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
    pub rfid: String, // rfid_tag: 123455623562354
    pub name: String, // name: Last, First
    pub doc: String,  // doc: 123345
    pub room: String,
    pub unit: usize,
}

impl Resident {
    pub fn new(rfid: String, name: String, doc: String, room: String, unit: usize) -> Self {
        Self {
            rfid,
            name,
            doc,
            room,
            unit,
        }
    }

    pub fn get_test_residents_from_file() -> Vec<Resident> {
        let file = std::fs::read_to_string("seed_data/residents.json").unwrap();
        serde_json::from_str::<Vec<Resident>>(&file).unwrap()
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateResident {
    pub rfid: Option<String>,
    pub name: Option<String>,
    pub doc: Option<String>,
    pub room: Option<String>,
    pub unit: Option<usize>,
}

impl UpdateResident {
    pub fn apply_to(self, resident: Resident) -> Resident {
        Resident {
            rfid: self.rfid.unwrap_or(resident.rfid),
            name: self.name.unwrap_or(resident.name),
            doc: self.doc.unwrap_or(resident.doc),
            room: self.room.unwrap_or(resident.room),
            unit: self.unit.unwrap_or(resident.unit),
        }
    }
}
