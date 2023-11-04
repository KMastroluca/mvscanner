use crate::database::db::DB;
use chrono::prelude::*;
use core::fmt::{Display, Formatter, Result};
use serde::{Deserialize, Serialize};
use serde_json::from_str;

#[derive(Debug, Serialize, Clone, Deserialize, Eq, PartialEq)]
pub struct TimeStamp {
    pub rfid: String,
    pub dest: String,
    pub time: String,
}

impl TimeStamp {
    pub fn new(rfid: String, dest: String, time: String) -> Self {
        Self { rfid, dest, time }
    }

    pub fn index(rfid: &str, db: &DB) -> Vec<Self> {
        db.index_timestamps(rfid, , end) 
    }

    pub fn bare(doc: usize, from: usize) -> Self {
        let now = Local::now();
        let date = now.format("%Y-%m-%d").to_string();
        let time = now.format("%H:%M:%S").to_string();
        Self {
            doc,
            to: 0,
            from,
            time: format!("{} @ {}", date, time),
        }
    }

    pub fn set_destination(&mut self, destination: usize) {
        self.to = destination;
    }

    pub fn is_today(&self) -> bool {
        let now = Local::now();
        let date = now.format("%Y-%m-%d").to_string();
        self.time.contains(&date)
    }
}

impl Display for TimeStamp {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} -> {} @ {}", self.from, self.to, self.time)
    }
}
