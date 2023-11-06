use crate::database::db::DB;
use actix_web::{
    middleware,
    web::{self, JsonConfig},
    App, HttpResponse, HttpServer,
};
use env_logger::{init_from_env, Env};
use scan_mvcf::{
    controllers::{locations_controller, residents_controller, timestamps_controller},
    database,
};
use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    init_from_env(Env::new().default_filter_or("info"));
    let pool = DB::default();
    if let Some(args) = std::env::args().nth(1) {
        match args.as_str() {
            "--seed" => {
                if pool.seed_test_data().is_ok() {
                    log::info!("database seeded with test data");
                } else {
                    log::info!("database seed failed");
                }
            }
            "--migrate" => {
                if pool.migrations().is_ok() {
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
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(json_config.clone())
            .service(locations_controller::index)
            .service(locations_controller::show)
            .service(locations_controller::show_location_range)
            .service(locations_controller::store)
            .service(residents_controller::index)
            .service(residents_controller::show)
            .service(residents_controller::show_resident_timestamps)
            .service(residents_controller::store)
            .service(residents_controller::destroy)
            .service(residents_controller::update)
            .service(timestamps_controller::index_timestamps)
            .service(timestamps_controller::show_range)
            .service(timestamps_controller::store_timestamp)
            .wrap(middleware::Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}
