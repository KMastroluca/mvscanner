use crate::models::locations::Location;
use crate::models::residents::Resident;
use crate::models::timestamps::TimeStamp;
use dirs::data_local_dir;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, Result};
use std::{path::PathBuf, thread};

#[derive(Debug)]
pub struct DB {
    conn: Pool<SqliteConnectionManager>,
}

impl Default for DB {
    fn default() -> Self {
        let path: PathBuf = data_local_dir().unwrap();
        Self::new(path)
    }
}

// name: String::new(),
// doc: String::new(),
// rfid_tag: String::new(),
// housing: Location::Delta(String::new()),
// signed_in: false,
// Location: None,
// timestamps: Vec::new(),
impl DB {
    pub fn new(path: PathBuf) -> Self {
        // If it doesn't exist, create it
        let dbpath = dirs::data_local_dir().unwrap().join("mvcf_scan.db");
        let manager = SqliteConnectionManager::file(dbpath);
        let pool = Pool::new(manager).unwrap();

        Self { conn: pool }
    }

    pub fn migrations(&self) -> Result<(), rusqlite::Error> {
        let mut locations = crate::models::locations::Location::read_from_file();

        self.conn
            .get()
            .unwrap()
            .execute(
                "CREATE TABLE IF NOT EXISTS locations (
                id INTEGER PRIMARY KEY NOT NULL,
                name TEXT NOT NULL,
                UNIQUE (id, name)
            )",
                params![],
            )
            .unwrap();

        self.conn
            .get()
            .unwrap()
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

        self.conn
            .get()
            .unwrap()
            .execute(
                "CREATE TABLE IF NOT EXISTS timestamps (
                id            INTEGER PRIMARY KEY AUTOINCREMENT,
                rfid          INTEGER NOT NULL,
                dest_id       INTEGER NOT NULL,
                timestamp     TEXT NOT NULL,
                ts            DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (doc) REFERENCES residents (doc),
                FOREIGN KEY (to_id) REFERENCES locations (id),
                FOREIGN KEY (from_id) REFERENCES locations (id)
            )",
                params![],
            )
            .unwrap();

        let _ = locations.iter().map(|i| {
            let pool = self.conn.clone();
            thread::spawn(move || {
                let conn = pool.get().unwrap();
                conn.execute(
                    "INSERT OR IGNORE INTO locations (id, name) VALUES (?1, ?2)",
                    params![&i.id, &i.name],
                )
                .unwrap();
            });
        });
        Ok(())
    }

    pub fn seed_test_data(&self) -> Result<(), rusqlite::Error> {
        let residents = Resident::get_test_residents_from_file();
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
        let mut stmt = self
            .conn
            .get()
            .unwrap()
            .prepare("SELECT * FROM residents")?;
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
        let mut stmt = self
            .conn
            .get()
            .unwrap()
            .prepare("SELECT * FROM residents WHERE rfid = ?1")?;
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
        if let Err(e) = self.conn.get().unwrap().execute(
            "INSERT INTO residents (rfid, name, doc, unit, room)
                VALUES (?1, ?2, ?3, ?4)",
            params![
                &resident.rfid_tag,
                &resident.name,
                &resident.doc,
                &resident.room,
            ],
        ) {
            println!("Error inserting resident {}", e);
            Err(e)
        } else {
            Ok(())
        };
        Ok(())
    }

    /// PUT: (Update) /api/residents/{id}
    pub fn update_resident(&self, resident: &Resident) -> Result<(), rusqlite::Error> {
        let mut stmt = self.conn.get().unwrap().prepare(
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
        let mut stmt = self
            .conn
            .get()
            .unwrap()
            .prepare("DELETE FROM residents WHERE rfid = ?1")?;
        stmt.execute(params![id])?;
        Ok(())
    }

    /// GET: (Update) /api/residents/{id}/timestamps/       DEFAULT: TODAY
    pub fn show_resident_timestamps(&self, rfid: String) -> Result<TimeStamp, rusqlite::Error> {
        let mut stmt = self
            .conn
            .get()
            .unwrap()
            .prepare("SELECT * FROM timestamps WHERE rfid = ?1 AND DATE(ts) = DATE('now')")?;
        let last = stmt.query_row(params![&rfid], |row| {
            Ok(TimeStamp::new(rfid, row.get(1)?, row.get(2)?))
        });
        if last.is_ok() {
            last
        } else {
            Err(rusqlite::Error::QueryReturnedNoRows)
        }
    }

    #[rustfmt::skip]
    pub fn show_resident_timestamps_range(&self, rfid: String, start: String, end: String) -> Result<Vec<TimeStamp>, rusqlite::Error> {
        let mut stmt = self.conn.get().unwrap().prepare(
            "SELECT * FROM timestamps WHERE rfid = ?1 AND DATE(ts) BETWEEN ?2 AND ?3")?;
        let timestamps_iter = stmt.query_map(params![&rfid, &start, &end], |row| {
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

    /// GET: (Show) /api/timestamps/{range}
    #[rustfmt::skip]
    pub fn index_timestamps_range(&self, start: String, end: String) -> Result<Vec<TimeStamp>, rusqlite::Error> {
        let mut stmt = self.conn.get().unwrap().prepare(
            "SELECT * FROM timestamps WHERE DATE(ts) BETWEEN ?2 AND ?3")?;
        let timestamps_iter = stmt.query_map(params![&start, &end], |row| {
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

    /// POST: (Store) /api/timestamps/{timestamp}
    pub fn store_timestamp(&self, ts: &TimeStamp) -> Result<(), rusqlite::Error> {
        let mut stmt = self.conn.get().unwrap().prepare(
            "INSERT INTO timestamps (rfid, to_id, time)
                  VALUES (?1, ?2, ?3)",
        )?;
        let _ = stmt.insert(params![&ts.rfid, &ts.dest, &ts.time])?;
        Ok(())
    }

    pub fn index_locations(&self) -> Result<Vec<Location>, rusqlite::Error> {
        let mut stmt = self
            .conn
            .get()
            .unwrap()
            .prepare("SELECT * FROM locations")?;
        let locations_iter =
            stmt.query_map([], |row| Ok(Location::new(row.get(0)?, row.get(1)?)))?;
        Ok(locations_iter
            .filter(|loc| loc.as_ref().is_ok())
            .map(|loc| loc.unwrap())
            .collect::<Vec<Location>>())
    }

    pub fn store_location(&self, loc: Location) -> Result<(), rusqlite::Error> {
        let mut stmt = self
            .conn
            .get()
            .unwrap()
            .prepare("INSERT OR IGNORE (id, name) INTO locaitons VALUES (?1, ?2)")?;
        if stmt.insert(params![loc.id, loc.name]).is_ok() {
            Ok(())
        } else {
            Err(rusqlite::Error::InvalidQuery)
        }
    }

    /// GET: (Index) /api/locations/{id}
    pub fn show_location(&self, id: String) -> Result<Location, rusqlite::Error> {
        let mut stmt = self
            .conn
            .get()
            .unwrap()
            .prepare("SELECT * FROM locations WHERE id = ?1 LIMIT 1")?;
        if let Ok(res) = stmt.query_row(params![id], |row| {
            Ok(Location::new(row.get_unwrap(0), row.get_unwrap(1)))
        }) {
            Ok(res)
        } else {
            Err(rusqlite::Error::QueryReturnedNoRows)
        }
    }
}
