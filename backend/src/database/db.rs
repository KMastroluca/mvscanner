use crate::models::locations::Location;
use crate::models::residents::Resident;
use crate::models::timestamps::TimeStamp;
use actix_web::{error, web};
use chrono::NaiveDate;
use rusqlite::{params, Result};
use serde::{Deserialize, Serialize};
pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;

pub type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;
#[derive(Debug, Clone, PartialEq)]
pub enum Query<'a> {
    IndexResidents,
    ShowResident(&'a str),
    StoreResident(&'a Resident),
    UpdateResident(&'a Resident),
    DestroyResident(String),
    ShowResidentTimestamps(String),
    ShowResidentTimestampsRange(&'a str, &'a NaiveDate, &'a NaiveDate),
    ShowLocationResidents(usize),
    UpdateResidentLocation(&'a Resident),
    IndexLocations,
    ShowLocation(usize),
    StoreLocation(&'a Location),
    ShowLocationTimestamps(usize),
    ShowLocationTimestampsRange(usize, &'a NaiveDate, &'a NaiveDate),
    IndexTimestamps,
    ShowTimestamps(&'a NaiveDate, &'a NaiveDate),
    StoreTimestamp(&'a TimeStamp),
    Migrations,
    SeedTestData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum QueryResult {
    Resident(Resident),
    Residents(Vec<Resident>),
    TimeStamps(Vec<TimeStamp>),
    Locations(Vec<Location>),
    Location(Location),
    Success,
    Failure,
    NotFound,
}

// name: String::new(),
// doc: String::new(),
// rfid_tag: String::new(),
// housing: Location::Delta(String::new()),
// signed_in: false,
// Location: None,
// timestamps: Vec::new(),
#[rustfmt::skip]
pub async fn query(pool: &Pool, query: Query<'_>,) -> Result<QueryResult, Box<dyn std::error::Error>> {
    let pool = pool.clone();
    let conn = web::block(move || pool.get())
        .await?
        .map_err(error::ErrorInternalServerError)?;
    match query {
        Query::ShowResident(id) => Ok(QueryResult::Resident(show_resident(id, conn)?)),
        Query::IndexResidents => Ok(QueryResult::Residents(index_residents(conn)?)),
        Query::StoreResident(resident) => {
            if store_resident(resident, conn).is_ok() {
                log::info!("Stored resident: {:?}", resident);
                Ok(QueryResult::Success)
            } else {
                log::info!("ERRORRRRRR: ");
                Err(Box::new(rusqlite::Error::InvalidQuery))
            }
        }
        Query::ShowResidentTimestampsRange(rfid, start, end) => {
            if let Ok(timestamps) = show_resident_timestamps_range(rfid, start, end, conn) {
            Ok(QueryResult::TimeStamps(timestamps))
            } else {
            Err(Box::new(rusqlite::Error::QueryReturnedNoRows))
            }
        }
        Query::UpdateResident(resident) => {
            if update_resident(resident, conn).is_ok() {
                Ok(QueryResult::Success)
            } else {
                Err(Box::new(rusqlite::Error::InvalidQuery))
            }
        }
        Query::ShowLocationResidents(id) => {
            if let Ok(residents) = show_location_residents(id, conn) {
                Ok(QueryResult::Residents(residents))
            } else {
                Err(Box::new(rusqlite::Error::QueryReturnedNoRows))
            }
        }
        Query::DestroyResident(id) => {
            if delete_resident(&id, conn).is_ok() {
                Ok(QueryResult::Success)
            } else {
                Err(Box::new(rusqlite::Error::InvalidQuery))
            }
        }
        Query::ShowResidentTimestamps(rfid) => {
            if let Ok(timestamps) = show_resident_timestamps(rfid, conn) {
                Ok(QueryResult::TimeStamps(timestamps))
            } else {
                Err(Box::new(rusqlite::Error::QueryReturnedNoRows))
            }
        }
        Query::UpdateResidentLocation(resident) => {
            if update_resident_location(resident, conn).is_ok() {
            Ok(QueryResult::Success)
            } else {
            Err(Box::new(rusqlite::Error::InvalidQuery))
            }
        }
        Query::IndexLocations => {
            if let Ok(locations) = index_locations(conn) {
                Ok(QueryResult::Locations(locations))
            } else {
                Ok(QueryResult::Failure)
            }
        }
        Query::ShowLocation(id) => {
            if let Ok(location) = show_location(id, conn) {
                Ok(QueryResult::Location(location))
            } else {
                Err(Box::new(rusqlite::Error::QueryReturnedNoRows))
            }
        }
        Query::StoreLocation(location) => {
            store_location(location, conn)?;
            Ok(QueryResult::Success)
        }
        Query::ShowLocationTimestampsRange(id, start, end) => Ok(QueryResult::TimeStamps(
            show_timestamps_location_range(id, start, end, conn)?,
        )),
        Query::ShowLocationTimestamps(id) => Ok(QueryResult::TimeStamps(
            show_timestamps_location(id, conn)?,
        )),
        Query::IndexTimestamps => Ok(QueryResult::TimeStamps(index_timestamps(conn)?)),
        Query::ShowTimestamps(start, end) => Ok(QueryResult::TimeStamps(show_timestamps_range(
            start, end, conn,
        )?)),
        Query::StoreTimestamp(ts) => {
            if  store_timestamp(ts, conn).is_ok() {
                Ok(QueryResult::Success)
            } else {
                Err(Box::new(rusqlite::Error::InvalidQuery))
            }
        }
        Query::Migrations => {
            if migrations(conn).is_ok() {
                Ok(QueryResult::Success)
            } else {
                Err(Box::new(rusqlite::Error::InvalidQuery))
            }
        }
        Query::SeedTestData => {
            if seed_test_data(conn).is_ok() {
                Ok(QueryResult::Success)
            } else {
                Err(Box::new(rusqlite::Error::InvalidQuery))
            }
        }
    }
}

pub fn migrations(conn: Connection) -> Result<(), Box<dyn std::error::Error>> {
    conn.execute_batch("BEGIN TRANSACTION;")?;

    let _ = conn.execute(
        "CREATE TABLE IF NOT EXISTS locations (
                id INTEGER PRIMARY KEY NOT NULL,
                name TEXT NOT NULL,
                UNIQUE (id, name)
            )",
        params![],
    );
    log::info!("Created locations table");

    let _ = conn
        .execute(
            "CREATE TABLE IF NOT EXISTS residents (
                rfid             TEXT PRIMARY KEY NOT NULL,
                name             TEXT NOT NULL,
                doc              TEXT NOT NULL,
                room             TEXT NOT NULL,
                unit             INTEGER NOT NULL,
                current_location INTEGER,
                FOREIGN KEY (unit) REFERENCES locations (id)
                FOREIGN KEY (current_location) REFERENCES locations (id)
                UNIQUE (rfid, name, doc)
                );",
            params![],
        )
        .unwrap();

    log::info!("Created residents table");
    let _ = conn
        .execute(
            "CREATE TABLE IF NOT EXISTS timestamps (
                    id            INTEGER PRIMARY KEY AUTOINCREMENT,
                    rfid          TEXT NOT NULL,
                    location      INTEGER NOT NULL,
                    ts            DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                    FOREIGN KEY (rfid) REFERENCES residents (rfid),
                    FOREIGN KEY (location) REFERENCES locations (id)
                );",
            params![],
        )
        .unwrap();

    log::info!("Created timestamps table");
    let locations = crate::models::locations::Location::read_from_file();

    for loc in locations {
        conn.execute(
            "INSERT OR IGNORE INTO locations (id, name) VALUES (?1, ?2)",
            params![&loc.id, &loc.name],
        )
        .unwrap();
    }
    log::info!("Inserted location data");
    conn.execute_batch("COMMIT;")?;
    Ok(())
}

pub fn seed_test_data(conn: Connection) -> Result<(), Box<dyn std::error::Error>> {
    let residents = Resident::get_test_residents_from_file();
    log::info!("SUCESS RESIDENTS");
    let timestamps = TimeStamp::get_test_timestamps_from_file()?;
    log::info!("SUCESS TIMESTAMPS");
    conn.execute_batch("BEGIN TRANSACTION;")?;

    for ts in timestamps {
        log::info!("Storing timestamp: {:?}", ts);
        conn.execute(
            "INSERT INTO timestamps (rfid, location)
                  VALUES (?1, ?2)",
            params![&ts.rfid, &ts.location],
        )?;
    }

    for resident in residents {
        log::info!("Storing resident: {:?}", resident);
        conn.execute(
            "INSERT INTO residents (rfid, name, doc, room, unit, current_location)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                &resident.rfid,
                &resident.name,
                &resident.doc,
                &resident.room,
                &resident.unit,
                &resident.current_location
            ],
        )?;
    }
    conn.execute_batch("COMMIT;")?; // Commit the transaction
    Ok(())
}

//
//-------------------------- RESIDENTS ---------------------------------//
//+++++=======================++++++===================================+++++

/// GET: (Index) /api/residents
fn index_residents(conn: Connection) -> Result<Vec<Resident>, Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare("SELECT * FROM residents")?;
    let residents_iter = stmt.query_map([], |row| {
        Ok(Resident::new(
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
            row.get(4)?,
            row.get(5)?,
        ))
    })?;

    Ok(residents_iter
        .filter_map(|res| res.is_ok().then(|| res.unwrap()))
        .collect::<Vec<Resident>>())
}

/// GET: (Show) /api/residents/{id}
fn show_resident(id: &str, conn: Connection) -> Result<Resident, Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare("SELECT * FROM residents WHERE rfid = ?1")?;
    if let Ok(resident) = stmt.query_row(params![id], |row| {
        Ok(Resident::new(
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
            row.get(4)?,
            row.get(5)?,
        ))
    }) {
        Ok(resident)
    } else {
        Err(Box::new(rusqlite::Error::QueryReturnedNoRows))
    }
}

/// POST: (Store) /api/residents/{resident}
#[rustfmt::skip]
fn store_resident(resident: &Resident, conn: Connection) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Storing resident: {:?}", resident);
    let query = "INSERT OR IGNORE INTO residents (rfid, name, doc, room, unit, current_location) VALUES (?1, ?2, ?3, ?4, ?5, ?6)";
    let mut stmt = conn.prepare(query)?;

    match stmt.execute(params![
        &resident.rfid,
        &resident.name,
        &resident.doc,
        &resident.room,
        &resident.unit,
        &resident.current_location,
    ]) {
        Ok(_) => Ok(()),
        Err(err) => {
            log::error!("Error executing SQL query: {}", err);
            Err(err.into())
        }
    }
}

/// PUT: (Update) /api/residents/{id}
#[rustfmt::skip]
  fn update_resident(resident: &Resident, conn: Connection) -> Result<(), Box<dyn std::error::Error>> {
        let mut stmt =
            conn.prepare("UPDATE residents SET name = ?2, doc = ?3, room = ?4, unit = ?5, current_location = ?6 WHERE rfid = ?1")?;
        stmt.execute(params![
            &resident.rfid,
            &resident.name,
            &resident.doc,
            &resident.room,
            &resident.unit,
            &resident.current_location
        ])?;
        Ok(())
    }

#[rustfmt::skip]
    /// DELETE: (Destroy) /api/residents/{id}
 fn delete_resident(id: &str, conn: Connection) -> Result<(), Box<dyn std::error::Error>> {
        let mut stmt = conn.prepare("DELETE FROM residents WHERE rfid = ?1")?;
        stmt.execute(params![id])?;
        Ok(())
    }

/// GET: (Update) /api/residents/{id}/timestamps/       DEFAULT: TODAY
    #[rustfmt::skip]
 fn show_resident_timestamps(rfid: String, conn: Connection) -> Result<Vec<TimeStamp>, Box<dyn std::error::Error>> {
        let mut stmt = conn
            .prepare("SELECT * FROM timestamps WHERE rfid = ?1 AND DATE(ts) = DATE('now')")?;
        let last_iter = stmt.query_map(params![&rfid.clone()], |row| {
            Ok(TimeStamp::new(row.get(1)?, row.get(2)?, row.get(3)?))
        })?;
        Ok(last_iter.map(|ts| ts.unwrap()).collect::<Vec<TimeStamp>>())
}

#[rustfmt::skip]
fn show_resident_timestamps_range(rfid: &str, start: &NaiveDate, end: &NaiveDate, conn: Connection) -> Result<Vec<TimeStamp>, Box<dyn std::error::Error>> {
    let start_date = start.format("%Y-%m-%d").to_string();
    let end_date = end.format("%Y-%m-%d").to_string();
    let mut stmt = conn
        .prepare("SELECT * FROM timestamps WHERE rfid = ?1 AND DATE(ts) BETWEEN DATE(?2) AND DATE(?3)")?;
    let last_iter = stmt.query_map(params![&rfid, &start_date, &end_date], |row| {
        Ok(TimeStamp::new(row.get(1)?, row.get(2)?, row.get(3)?))
    })?;
    Ok(last_iter.map(|ts| ts.unwrap()).collect::<Vec<TimeStamp>>())
}

// +++++========================+++++==========================================++++++
/// ----------------------------- TIMESTAMPS ---------------------------------///

/// GET: (Index) /api/timestamps/
#[rustfmt::skip]
fn index_timestamps(conn: Connection) -> Result<Vec<TimeStamp>, Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare("SELECT * FROM timestamps WHERE DATE(ts) = DATE('now')")?;
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
 fn show_timestamps_range(start: &NaiveDate, end: &NaiveDate, conn: Connection) -> Result<Vec<TimeStamp>, Box<dyn std::error::Error>> {
    log::info!("Fetching timestamps between {} and {}", start, end);
        let start = start.format("%Y-%m-%d").to_string();
        let end = end.format("%Y-%m-%d").to_string();
        let mut stmt = conn.prepare(
            "SELECT * FROM timestamps WHERE DATE(ts) BETWEEN DATE(?1) AND DATE(?2)")?;
        let timestamps_iter = stmt.query_map(params![&start, &end], |row| {
            Ok(TimeStamp::new(
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
            ))
        })?;
    Ok(timestamps_iter
        .filter_map(|ts| ts.is_ok().then(|| ts.unwrap()))
        .collect::<Vec<TimeStamp>>())
    }

/// GET: (Show) /api/locations/{id}/timestamps
#[rustfmt::skip]
 fn show_timestamps_location(id: usize, conn: Connection) -> Result<Vec<TimeStamp>, Box<dyn std::error::Error>> {
        let mut stmt = conn.prepare(
                "SELECT * FROM timestamps WHERE location = ?1 AND DATE(ts) = DATE('now')",
            )?;
        let timestamps_iter = stmt.query_map(params![&id], |row| {
            Ok(TimeStamp::new(row.get(1)?, row.get(2)?, row.get(3)?))
        })?;
        Ok(timestamps_iter
            .filter_map(|ts| ts.is_ok().then(|| ts.unwrap()))
            .collect::<Vec<TimeStamp>>())
    }

/// GET: (Show) /api/locations/{id}/timestamps/{start}/{end}
#[rustfmt::skip]
 fn show_timestamps_location_range(id: usize, start: &NaiveDate, end: &NaiveDate, conn: Connection) -> Result<Vec<TimeStamp>, Box<dyn std::error::Error>> {
    log::info!("Fetching timestamps between {} and {}", start, end);
        let start = start.format("%Y-%m-%d").to_string();
        let end = end.format("%Y-%m-%d").to_string();
        let mut stmt = conn.prepare(
                "SELECT * FROM timestamps WHERE location = ?1 AND DATE(ts) BETWEEN DATE(?2) AND DATE(?3)",
            )?;
        let timestamps_iter = stmt.query_map(params![&id, &start, &end], |row| {
            Ok(TimeStamp::new(row.get(1)?, row.get(2)?, row.get(3)?))
        })?;
        Ok(timestamps_iter
            .filter_map(|ts| ts.is_ok().then(|| ts.unwrap()))
            .collect::<Vec<TimeStamp>>())
    }

#[rustfmt::skip]
fn update_resident_location(resident: &Resident, conn: Connection) -> Result<(), Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare("UPDATE residents SET current_location = ?1 WHERE rfid = ?2")?;
    stmt.execute(params![&resident.current_location, &resident.rfid])?;
  // let mut stmt = conn.prepare("SELECT * FROM residents WHERE rfid = ?1")?;
  //   let mut res = stmt.query_row(params![&ts.rfid], |row| {
  //       Ok(Resident::new(
  //           row.get(0)?,
  //           row.get(1)?,
  //           row.get(2)?,
  //           row.get(3)?,
  //           row.get(4)?,
  //           row.get(5)?,
  //       ))
  //   })?;
  //   res.update_location(ts.location);
  //
  //   let mut update_stmt = conn.prepare(
  //       "UPDATE residents SET name = ?2, doc = ?3, room = ?4, unit = ?5, current_location = ?6 WHERE rfid = ?1",
  //   )?;
  //   update_stmt.execute(params![
  //       &res.rfid,
  //       &res.name,
  //       &res.doc,
  //       &res.room,
  //       &res.unit,
  //       &res.current_location,
  //   ])?;
    Ok(())
}

#[rustfmt::skip]
fn store_timestamp(ts: &TimeStamp, conn: Connection) -> Result<(), Box<dyn std::error::Error>> {
  //
  //   // Insert the timestamp
    let mut stmt = conn.prepare(
        "INSERT INTO timestamps (rfid, location)
                  VALUES (?1, ?2)",
    )?;
    stmt.insert(params![&ts.rfid, &ts.location])?;

    Ok(())
}

fn index_locations(conn: Connection) -> Result<Vec<Location>, Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare("SELECT * FROM locations")?;
    let locations_iter = stmt.query_map([], |row| Ok(Location::new(row.get(0)?, row.get(1)?)))?;
    log::info!("Locations fetched!");
    Ok(locations_iter
        .filter(|loc| loc.as_ref().is_ok())
        .map(|loc| loc.unwrap())
        .collect::<Vec<Location>>())
}

fn store_location(loc: &Location, conn: Connection) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Storing location: {:?}", loc);
    let mut stmt = conn.prepare("INSERT OR IGNORE INTO locations (id, name) VALUES (?1, ?2)")?;
    if stmt.execute(params![&loc.id, &loc.name]).is_ok() {
        Ok(())
    } else {
        Err(Box::new(rusqlite::Error::InvalidQuery))
    }
}

/// GET: (Index) /api/locations/{id}
fn show_location(id: usize, conn: Connection) -> Result<Location, Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare("SELECT * FROM locations WHERE id = ?1 LIMIT 1")?;
    if let Ok(res) = stmt.query_row(params![&id], |row| {
        Ok(Location::new(row.get_unwrap(0), row.get_unwrap(1)))
    }) {
        Ok(res)
    } else {
        Err(Box::new(rusqlite::Error::QueryReturnedNoRows))
    }
}

/// GET: (Show) /api/locations/{id}/residents
#[rustfmt::skip]
fn show_location_residents(id: usize, conn: Connection) -> Result<Vec<Resident>, Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare("SELECT * FROM residents WHERE unit = ?1")?;
    let residents_iter = stmt.query_map(params![&id], |row| {
        Ok(Resident::new(
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
            row.get(4)?,
            row.get(5)?,
        ))
    })?;
    Ok(residents_iter
        .filter_map(|res| res.is_ok().then(|| res.unwrap()))
        .collect::<Vec<Resident>>())
}
