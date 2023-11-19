use serde::{de::Error, Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize, Eq, PartialEq)]
pub struct PostTimestamp {
    pub rfid: String,
    pub dest: usize,
}

#[derive(Debug, Serialize, Clone, Deserialize, Eq, PartialEq)]
pub struct TimeStamp {
    pub rfid: String,
    pub dest: usize,
    pub time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct Range {
    /// ISO start date (YYYY-MM-DD)
    pub start: String,
    /// ISO end date (YYYY-MM-DD)
    pub end: String,
}

impl TimeStamp {
    pub fn new(rfid: String, dest: usize, time: Option<String>) -> Self {
        Self { rfid, dest, time }
    }
    pub fn get_test_timestamps_from_file() -> Result<Vec<TimeStamp>, serde_json::Error> {
        if let Ok(ts_str) = std::fs::read_to_string("seed_data/timestamps.json") {
            serde_json::from_str::<Vec<TimeStamp>>(&ts_str)
        } else {
            Err(serde_json::Error::custom("Unable to read file"))
        }
    }
}
