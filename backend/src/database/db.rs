use crate::models::residents::Resident;
use crate::models::timestamps::TimeStamp;
use crate::models::{locations::Location, timestamps::Range};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, Result};
#[derive(Debug, Clone)]
pub struct DB {
    conn: Pool<SqliteConnectionManager>,
}

impl Default for DB {
    fn default() -> Self {
        Self::new()
    }
}

pub enum Query<'a> {
    IndexResidents,
    ShowResident(String),
    StoreResident(&'a Resident),
    UpdateResident(&'a Resident),
    DeleteResident(String),
    ShowResidentTimestamps(String),
    IndexLocations,
    ShowLocation(usize),
    StoreLocation(&'a Location),
    ShowLocationTimestamps(usize, &'a Range),
    IndexTimestamps,
    ShowTimestamps(&'a Range),
}

pub enum QueryResult {
    Resident(Resident),
    TimeStamps(Vec<TimeStamp>),
    Residents(Vec<Resident>),
    Locations(Vec<Location>),
    Location(Location),
}

// name: String::new(),
// doc: String::new(),
// rfid_tag: String::new(),
// housing: Location::Delta(String::new()),
// signed_in: false,
// Location: None,
// timestamps: Vec::new(),
impl DB {
    pub fn new() -> Self {
        // If it doesn't exist, create it
        let dbpath = dirs::data_local_dir().unwrap().join("mvcf_scan.db");
        let manager = SqliteConnectionManager::file(dbpath);
        let pool = Pool::new(manager).unwrap();

        Self { conn: pool }
    }
    pub fn query(&self, query: Query) -> Result<QueryResult, rusqlite::Error> {
        match query {
            Query::IndexResidents => {
                let residents = self.index_residents()?;
                Ok(QueryResult::Residents(residents))
            }
            Query::ShowResident(id) => {
                let resident = self.show_resident(&id)?;
                Ok(QueryResult::Resident(resident))
            }
            Query::StoreResident(resident) => {
                self.store_resident(resident)?;
                Ok(QueryResult::Residents(vec![resident.clone()]))
            }
            Query::UpdateResident(resident) => {
                self.update_resident(resident)?;
                Ok(QueryResult::Resident(resident.clone()))
            }
            Query::DeleteResident(id) => {
                self.delete_resident(&id)?;
                Ok(QueryResult::Residents(Vec::new()))
            }
            Query::ShowResidentTimestamps(rfid) => {
                let timestamps = self.show_resident_timestamps(rfid)?;
                Ok(QueryResult::TimeStamps(timestamps))
            }
            Query::IndexLocations => {
                let locations = self.index_locations()?;
                Ok(QueryResult::Locations(locations))
            }
            Query::ShowLocation(id) => {
                let location = self.show_location(id)?;
                Ok(QueryResult::Locations(vec![location]))
            }
            Query::StoreLocation(location) => {
                self.store_location(location)?;
                Ok(QueryResult::Locations(vec![location.clone()]))
            }
            Query::ShowLocationTimestamps(id, range) => {
                let timestamps =
                    self.show_timestamps_location_range(id, &range.start, &range.end)?;
                Ok(QueryResult::TimeStamps(timestamps))
            }
            Query::IndexTimestamps => {
                let timestamps = self.index_timestamps()?;
                Ok(QueryResult::TimeStamps(timestamps))
            }
            Query::ShowTimestamps(range) => {
                let timestamps = self.index_timestamps_range(range)?;
                Ok(QueryResult::TimeStamps(timestamps))
            }
        }
    }
    pub fn migrations(&self) -> Result<(), rusqlite::Error> {
        let get_conn = self.conn.get().unwrap();
        let _ = get_conn
            .execute(
                "CREATE TABLE IF NOT EXISTS locations (
                id INTEGER PRIMARY KEY NOT NULL,
                name TEXT NOT NULL,
                UNIQUE (id, name)
            )",
                params![],
            )
            .unwrap();
        log::info!("Created locations table");

        let _ = get_conn
            .execute(
                "CREATE TABLE IF NOT EXISTS residents (
                rfid            INTEGER PRIMARY KEY NOT NULL,
                name            TEXT NOT NULL,
                doc             INTEGER NOT NULL,
                unit            INTEGER NOT NULL,
                room            TEXT NOT NULL,
                FOREIGN KEY (unit) REFERENCES locations (id)
            )",
                params![],
            )
            .unwrap();

        log::info!("Created residents table");
        self.conn
            .get()
            .unwrap()
            .execute(
                "CREATE TABLE IF NOT EXISTS timestamps (
                    id            INTEGER PRIMARY KEY AUTOINCREMENT,
                    rfid          INTEGER NOT NULL,
                    dest          INTEGER NOT NULL,
                    timestamp     TEXT NOT NULL,
                    ts            DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                    FOREIGN KEY (rfid) REFERENCES residents (rfid),
                    FOREIGN KEY (dest) REFERENCES locations (id)
                );",
                params![],
            )
            .unwrap();

        log::info!("Created timestamps table");
        let locations = crate::models::locations::Location::read_from_file();
        let _ = locations.iter().map(|i| {
            let pool = self.conn.clone();
            let conn = pool.get().unwrap();
            conn.execute(
                "INSERT OR IGNORE INTO locations (id, name) VALUES (?1, ?2)",
                params![&i.id.clone(), &i.name.clone()],
            )
            .unwrap();
        });
        log::info!("Entered data into locations table");
        Ok(())
    }

    pub fn seed_test_data(&self) -> Result<(), rusqlite::Error> {
        let residents = Resident::get_test_residents_from_file();
        if let Ok(timestamps) = TimeStamp::get_test_timestamps_from_file() {
            for timestamp in timestamps {
                if let Err(e) = self.store_timestamp(&timestamp) {
                    log::info!("Error storing test timestamps: {}", e);
                }
            }
        }
        for resident in residents {
            if let Err(e) = self.store_resident(&resident) {
                println!("Error inserting resident: {:?}", e);
            }
        }
        Ok(())
    }

    //
    //-------------------------- RESIDENTS ---------------------------------//
    //+++++=======================++++++===================================+++++

    /// GET: (Index) /api/residents
    pub fn index_residents(&self) -> Result<Vec<Resident>, rusqlite::Error> {
        let get_conn = self.conn.get().unwrap();
        let mut stmt = get_conn.prepare("SELECT * FROM residents")?;
        let residents_iter = stmt.query_map([], |row| {
            Ok(Resident::new(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
            ))
        })?;

        Ok(residents_iter
            .map(|res| res.unwrap())
            .collect::<Vec<Resident>>())
    }

    /// GET: (Show) /api/residents/{id}
    pub fn show_resident(&self, id: &str) -> Result<Resident, rusqlite::Error> {
        let get_conn = self.conn.get().unwrap();
        let mut stmt = get_conn.prepare("SELECT * FROM residents WHERE rfid = ?1")?;
        if let Ok(resident) = stmt.query_row(params![id], |row| {
            Ok(Resident::new(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
            ))
        }) {
            Ok(resident)
        } else {
            Err(rusqlite::Error::QueryReturnedNoRows)
        }
    }

    /// POST: (Store) /api/residents/{resident}
    pub fn store_resident(&self, resident: &Resident) -> Result<(), rusqlite::Error> {
        log::info!("Storing resident: {:?}", resident);
        if let Ok(get_conn) = self.conn.get() {
            let _ = get_conn.execute(
                "INSERT INTO residents (rfid, name, doc, unit, room)
                VALUES (?1, ?2, ?3, ?4)",
                params![
                    &resident.rfid_tag,
                    &resident.name,
                    &resident.doc,
                    &resident.room,
                ],
            );
        }
        Ok(())
    }

    /// PUT: (Update) /api/residents/{id}
    pub fn update_resident(&self, resident: &Resident) -> Result<(), rusqlite::Error> {
        let get_conn = self.conn.get().unwrap();
        let mut stmt = get_conn.prepare(
            "UPDATE residents SET name = ?1, doc = ?2, unit = ?3, room = ?4 WHERE rfid = ?5",
        )?;
        stmt.execute(params![
            &resident.rfid_tag,
            &resident.name,
            &resident.doc,
            &resident.room,
        ])?;
        Ok(())
    }
    /// DELETE: (Destroy) /api/residents/{id}
    pub fn delete_resident(&self, id: &str) -> Result<(), rusqlite::Error> {
        let get_conn = self.conn.get().unwrap();
        let mut stmt = get_conn.prepare("DELETE FROM residents WHERE rfid = ?1")?;
        stmt.execute(params![id])?;
        Ok(())
    }

    /// GET: (Update) /api/residents/{id}/timestamps/       DEFAULT: TODAY
    pub fn show_resident_timestamps(
        &self,
        rfid: String,
    ) -> Result<Vec<TimeStamp>, rusqlite::Error> {
        let get_conn = self.conn.get().unwrap();
        let mut stmt = get_conn
            .prepare("SELECT * FROM timestamps WHERE rfid = ?1 AND DATE(ts) = DATE('now')")?;
        let last_iter = stmt.query_map(params![&rfid.clone()], |row| {
            Ok(TimeStamp::new(row.get(1)?, row.get(2)?, row.get(3)?))
        })?;
        Ok(last_iter.map(|ts| ts.unwrap()).collect::<Vec<TimeStamp>>())
    }

    #[rustfmt::skip]
    pub fn show_resident_timestamps_range(&self, rfid: &String, range: &Range) -> Result<Vec<TimeStamp>, rusqlite::Error> {
        let get_conn = self.conn.get().unwrap();
        let mut stmt = get_conn.prepare(
            "SELECT * FROM timestamps WHERE rfid = ?1 AND DATE(ts) BETWEEN ?2 AND ?3")?;
        let timestamps_iter = stmt.query_map(params![&rfid, &range.start, &range.end], |row| {
            Ok(TimeStamp::new(
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
            ))
        })?;
        let mut timestamps = Vec::new();
        let _ = timestamps_iter
            .filter(|ts| ts.as_ref().is_ok())
            .map(|x| timestamps.push(x.unwrap()));
        if timestamps.is_empty() {
            Err(rusqlite::Error::QueryReturnedNoRows)
        } else {
         Ok(timestamps)
        }
    }

    // +++++========================+++++==========================================++++++
    /// ------------------ TIMESTAMPS ------------------------///

    /// GET: (Index) /api/timestamps/
    pub fn index_timestamps(&self) -> Result<Vec<TimeStamp>, rusqlite::Error> {
        let get_conn = self.conn.get().unwrap();
        let mut stmt = get_conn.prepare("SELECT * FROM timestamps WHERE DATE(ts) = DATE('now')")?;
        let timestamps = stmt.query_map([], |row| {
            Ok(TimeStamp::new(row.get(1)?, row.get(2)?, row.get(3)?))
        })?;
        Ok(timestamps
            .filter(|ts| ts.as_ref().is_ok())
            .map(|x| x.unwrap())
            .collect::<Vec<TimeStamp>>())
    }

    /// GET: (Show) /api/timestamps/{range}
    #[rustfmt::skip]
    pub fn index_timestamps_range(&self, range: &Range) -> Result<Vec<TimeStamp>, rusqlite::Error> {
        let get_conn = self.conn.get().unwrap();
        let mut stmt = get_conn.prepare(
            "SELECT * FROM timestamps WHERE DATE(ts) BETWEEN ?2 AND ?3")?;
        let timestamps_iter = stmt.query_map(params![&range.start, &range.end], |row| {
            Ok(TimeStamp::new(
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
            ))
        })?;
        let mut timestamps = Vec::new();
        let _ = timestamps_iter
            .filter(|ts| ts.as_ref().is_ok())
            .map(|x| timestamps.push(x.unwrap()));
        if timestamps.is_empty() {
            Err(rusqlite::Error::QueryReturnedNoRows)
        } else {
         Ok(timestamps)
        }
    }

    /// GET: (Show) /api/locations/{id}/timestamps/{range}
#[rustfmt::skip]
    pub fn show_timestamps_location_range(&self, id: usize, start: &String, end: &String) -> Result<Vec<TimeStamp>, rusqlite::Error> {
        let get_conn =
            self.conn.get().unwrap();
        let mut stmt = get_conn.prepare(
                "SELECT * FROM timestamps WHERE dest = ?1 AND DATE(ts) BETWEEN ?2 AND ?3",
            )?;
        let timestamps_iter = stmt.query_map(params![&id, &start, &end], |row| {
            Ok(TimeStamp::new(row.get(1)?, row.get(2)?, row.get(3)?))
        })?;
        Ok(timestamps_iter
            .filter_map(|ts| ts.is_ok().then(|| ts.unwrap()))
            .collect::<Vec<TimeStamp>>())
    }

    /// POST: (Store) /api/timestamps/{timestamp}
    pub fn store_timestamp(&self, ts: &TimeStamp) -> Result<(), rusqlite::Error> {
        let get_conn = self.conn.get().unwrap();
        let mut stmt = get_conn.prepare(
            "INSERT INTO timestamps (rfid, dest, timestamp)
                  VALUES (?1, ?2, ?3)",
        )?;
        let _ = stmt.insert(params![&ts.rfid, &ts.dest, &ts.time])?;
        Ok(())
    }

    pub fn index_locations(&self) -> Result<Vec<Location>, rusqlite::Error> {
        let get_conn = self.conn.get().unwrap();
        let mut stmt = get_conn.prepare("SELECT * FROM locations")?;
        let locations_iter =
            stmt.query_map([], |row| Ok(Location::new(row.get(0)?, row.get(1)?)))?;
        log::info!("Locations fetched!");
        Ok(locations_iter
            .filter(|loc| loc.as_ref().is_ok())
            .map(|loc| loc.unwrap())
            .collect::<Vec<Location>>())
    }

    pub fn store_location(&self, loc: &Location) -> Result<(), rusqlite::Error> {
        log::info!("Storing location: {:?}", loc);
        let get_conn = self.conn.get().unwrap();
        let mut stmt =
            get_conn.prepare("INSERT OR IGNORE (id, name) INTO locations VALUES (?1, ?2)")?;
        if stmt.insert(params![loc.id, loc.name]).is_ok() {
            Ok(())
        } else {
            Err(rusqlite::Error::InvalidQuery)
        }
    }

    /// GET: (Index) /api/locations/{id}
    pub fn show_location(&self, id: usize) -> Result<Location, rusqlite::Error> {
        let get_conn = self.conn.get().unwrap();
        let mut stmt = get_conn.prepare("SELECT * FROM locations WHERE id = ?1 LIMIT 1")?;
        if let Ok(res) = stmt.query_row(params![&id], |row| {
            Ok(Location::new(row.get_unwrap(0), row.get_unwrap(1)))
        }) {
            Ok(res)
        } else {
            Err(rusqlite::Error::QueryReturnedNoRows)
        }
    }
}
