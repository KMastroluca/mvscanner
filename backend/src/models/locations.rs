use chrono::prelude::*;
use core::fmt::{Display, Formatter, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env::current_dir;
use std::path::PathBuf;

#[derive(Serialize, Clone, Deserialize, Debug, Eq, PartialEq)]
pub struct Housing {
    unit: usize,
    pod_room: String,
}

#[derive(Serialize, Clone, Deserialize, Debug, Default, Eq, PartialEq)]
pub struct Location {
    pub id: usize,
    pub name: String,
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}: {}", self.id, self.name)
    }
}

impl From<Location> for (usize, String) {
    fn from(loc: Location) -> Self {
        (loc.id, loc.name)
    }
}

impl Location {
    pub fn new(id: usize, name: String) -> Self {
        Self { id, name }
    }
    pub fn get_env() -> Self {
        Self {
            id: std::env::var("LOCATION_ID")
                .unwrap_or(String::from("14"))
                .parse::<usize>()
                .unwrap_or(14),
            name: std::env::var("LOCATION_NAME").unwrap_or(String::from("DELTA UNIT")),
        }
    }

    pub fn read_from_file() -> Vec<Location> {
        let path: PathBuf = if std::env::var("SCAN_LOCATIONS").is_ok() {
            std::env::var("SCAN_LOCATIONS").unwrap().into()
        } else {
            current_dir().unwrap().join("locations.json")
        };
        let file: String = std::fs::read_to_string(path).unwrap();
        let loc: Vec<Location> = serde_json::from_str::<Vec<Location>>(&file).unwrap();
        loc
    }

    pub fn all() -> HashMap<usize, String> {
        let mut map = HashMap::new();
        for location in Location::read_from_file().iter() {
            map.insert(location.id, location.name.clone());
        }
        map
    }

    pub fn get_location_by_id(id: usize) -> Option<Location> {
        if let Some(loc) = Location::all().get(&id) {
            return Some(Location::new(id, String::from(loc)));
        }
        None
    }
}

impl Default for Housing {
    fn default() -> Self {
        Self {
            pod_room: "".to_string(),
            unit: 0,
        }
    }
}

impl Display for Housing {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {}", self.unit, self.pod_room)
    }
}

impl Housing {
    pub fn new(pod_room: &str, unit: usize) -> Self {
        Self {
            pod_room: String::from(pod_room),
            unit,
        }
    }

    pub fn get_unit(&self) -> usize {
        self.unit
    }

    pub fn get_room(&self) -> &str {
        self.pod_room.as_str()
    }
}

pub fn validate_room(room: &str) -> bool {
    // A-1B
    let mut splt = room.split('-');
    splt.next()
        .is_some_and(|x| is_unit_digit(x.to_uppercase().chars().next().unwrap()))
        && splt.next().is_some_and(|x| {
            x.to_ascii_uppercase()
                .chars()
                .next()
                .unwrap()
                .is_ascii_digit()
        })
}

fn is_unit_digit(unit: char) -> bool {
    matches!(unit, 'A'..='D')
}

/*
const ACTIVITIES_ID: i32 = 2;
const ALPHA_UNIT_ID: i32 = 1;
const CHAPEL_ID: i32 = 3;
const ASU_ID: i32 = 4;
const BOOKING_ID: i32 = 5;
const BRAVO_UNIT_ID: i32 = 6;
const CHARLIE_UNIT_ID: i32 = 7;
const CHARLIE_UNIT_CLASSROOM_ID: i32 = 8;
const CHAPLAINS_OFFICE_ID: i32 = 9;
const COLLEGE_CLASSROOM_ID: i32 = 10;
const COMPUTER_LAB_ID: i32 = 11;
const CULINARY_ID: i32 = 12;
const D_BOARDS_ID: i32 = 13;
const DELTA_UNIT_ID: i32 = 14;
const DELTA_UNIT_CLASSROOM_ID: i32 = 15;
const EDUCATION_CONFERENCE_ROOM_ID: i32 = 16;
const ECHO_UNIT_ID: i32 = 17;
const FLOOR_JANITOR_ID: i32 = 18;
const GYM_ID: i32 = 19;
const MCDONALD_ID: i32 = 20;
const HOSPITAL_ID: i32 = 21;
const KITCHEN_ID: i32 = 22;
const LAUNDRY_ID: i32 = 23;
const LIBRARY_ID: i32 = 24;
const MEDICAL_ID: i32 = 25;
const MUSIC_ROOM_ID: i32 = 26;
const NCCER_ID: i32 = 27;
const OFF_GROUNDS_ID: i32 = 28;
const OUTSIDE_RECREATION_ID: i32 = 30;
const VISIT_ROOM_ID: i32 = 31;
const SMALL_ENGINES_ID: i32 = 32;
const WOOD_SHOP_ID: i32 = 33;
const WORK_CREW_ID: i32 = 34;
const CASEWORKER_HEAL_ID: i32 = 35;
const STAFF_JACKSON_ID: i32 = 36;
const UM_HARMON_ID: i32 = 37;
const STAFF_FRENCH_ID: i32 = 39;
const ANNEX_ID: i32 = 41;
const CASEWORKER_DEVER_JACOB_ID: i32 = 42;
*/
