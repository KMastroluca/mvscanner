use crate::database::db::DB;
use serde::{de::Error, Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize, Eq, PartialEq)]
pub struct TimeStamp {
    pub rfid: String,
    pub dest: usize,
    pub time: String,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct Range {
    /// ISO start date (YYYY-MM-DD)
    pub start: String,
    /// ISO end date (YYYY-MM-DD)
    pub end: String,
}

impl TimeStamp {
    pub fn new(rfid: String, dest: usize, time: String) -> Self {
        Self { rfid, dest, time }
    }
    pub fn get_test_timestamps_from_file() -> Result<Vec<TimeStamp>, serde_json::Error> {
        if let Ok(ts_str) = std::fs::read_to_string("seed_data/timestamps.json") {
            serde_json::from_str::<Vec<TimeStamp>>(&ts_str)
        } else {
            Err(serde_json::Error::custom("Unable to read file"))
        }
    }

    /// GET: /api/timestamps
    pub fn index(db: &DB) -> Vec<Self> {
        db.index_timestamps().unwrap_or_default()
    }
    /// POST: /api/timestamps{body}
    pub fn store(db: &DB, timestamp: &TimeStamp) -> Result<(), rusqlite::Error> {
        db.store_timestamp(timestamp)
    }

    /// GET: /api/timestamps/{rfid}
    pub fn show_range_resident(db: &DB, rfid: &String, range: &Range) -> Vec<Self> {
        db.show_resident_timestamps_range(rfid, range)
            .unwrap_or_default()
    }

    pub fn index_range(db: &DB, range: &Range) -> Vec<Self> {
        db.index_timestamps_range(range).unwrap_or_default()
    }

    pub fn show_location_range(db: &DB, id: usize, range: &Range) -> Vec<Self> {
        db.show_timestamps_location_range(id, &range.start, &range.end)
            .unwrap_or_default()
    }
}
