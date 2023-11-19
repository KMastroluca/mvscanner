use actix_cors::Cors;
use actix_web::{
    middleware,
    web::{Data, JsonConfig},
    App, HttpServer,
};
use r2d2_sqlite::SqliteConnectionManager;
use scan_mvcf::{
    controllers::{locations_controller, residents_controller, timestamps_controller},
    database::db::{query, Query},
};
use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let dbpath = dirs::data_local_dir().unwrap().join("mvcf_scan.db");
    let manager = SqliteConnectionManager::file(dbpath);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Not pointing to proper file");
    if let Some(args) = std::env::args().nth(1) {
        match args.as_str() {
            "--test-seed" => {
                if query(&pool, Query::Migrations).await.is_ok() {
                    log::info!("database migrations complete");
                    if query(&pool, Query::SeedTestData).await.is_ok() {
                        log::info!("database seeded with test data");
                    } else {
                        log::info!("database seed failed");
                    }
                } else {
                    log::info!("database migrations failed");
                }
            }
            "--migrate" => {
                if query(&pool, Query::Migrations).await.is_ok() {
                    log::info!("database migrations complete");
                } else {
                    log::info!("database migrations failed");
                }
            }
            &_ => {
                log::info!("invalid argument: {}", args);
            }
        }
    }
    log::info!("starting Actix-Web HTTP server at http://localhost:8080");
    let json_config = JsonConfig::default().limit(4096);
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .max_age(3600);

        App::new()
            .app_data(Data::new(pool.clone()))
            .app_data(json_config.clone())
            .service(locations_controller::index)
            .service(locations_controller::show)
            .service(locations_controller::show_location_timestamps)
            .service(locations_controller::show_location_timestamps_range)
            .service(locations_controller::show_location_residents)
            .service(locations_controller::store)
            .service(residents_controller::index)
            .service(residents_controller::show)
            .service(residents_controller::show_resident_timestamps)
            .service(residents_controller::show_resident_timestamps_range)
            .service(residents_controller::store)
            .service(residents_controller::destroy)
            .service(residents_controller::update)
            .service(timestamps_controller::index_timestamps)
            .service(timestamps_controller::show_range)
            .service(timestamps_controller::store_timestamp)
            .wrap(middleware::Logger::default())
            .wrap(cors)
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}
