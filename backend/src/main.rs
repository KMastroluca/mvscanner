use crate::database::db::{Pool, DB};
use actix_web::{middleware, web, App, Error as AWError, HttpRequest, HttpServer, Result};
use r2d2_sqlite::{self, SqliteConnectionManager};
use scan_mvcf::{
    controllers::{locations_controller, timestamps_controller},
    database,
};

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let pool = database::db::DB::default();

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool))
            .configure(app_config::config_app)
            .wrap(middleware::Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}
