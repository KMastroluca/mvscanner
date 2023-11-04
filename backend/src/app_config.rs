use crate::controllers;
use crate::controllers::locations_controller;
use crate::controllers::residents_controller;
use crate::controllers::timestamps_controller;
use crate::database::db::DB;
use crate::models::timestamps;
use actix_web::web;
use actix_web::HttpRequest;

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("api/residents")
            .service(
                web::resource("")
                    .route(web::get().to(residents_controller::index.into()))
                    .route(web::post().to(residents_controller::store.into())), // Handle POST request for /api/residents
            )
            .service(
                web::resource("/{id}")
                    .route(web::get().to(controllers::residents_controller::show.into())) // Handle GET request for /api/residents/{id}
                    .route(web::put().to(residents_controller::update.into())) // Handle PUT request for /api/residents/{id}
                    .route(web::delete().to(controllers::residents_controller::destroy.into())), // Handle DELETE request for /api/residents/{id}
            )
            .service(
                web::resource("/{id}/timestamps")
                    .route(web::get().to(timestamps_controller:)), // Handle GET request for /api/residents/{id}/timestamps
            ),
    );

    // Locations API
    let locations_index_handler = |db: web::Data<DB>| {
        |req: HttpRequest| {
            let locations = locations_controller::index(&db);
            // Process 'locations' and return a response as needed
        }
    };

    let locations_store_handler = |db: web::Data<DB>| {
        |req: HttpRequest| {
            let result = locations_controller::store(&db, req);
            // Process 'result' and return a response as needed
        }
    };

    let locations_show_handler = |db: web::Data<DB>| {
        |req: HttpRequest| {
            let location_id = req.match_info().get("id").unwrap();
            let location = locations_controller::show(&db, location_id);
            // Process 'location' and return a response as needed
        }
    };

    let locations_get_timestamps_handler = |db: web::Data<DB>| {
        |req: HttpRequest| {
            let location_id = req.match_info().get("id").unwrap();
            let timestamps = locations_controller::get_timestamps(&db, location_id);
            // Process 'timestamps' and return a response as needed
        }
    };

    // Timestamps API
    let timestamps_index_handler = |db: web::Data<DB>| {
        |req: HttpRequest| {
            let timestamps = timestamps_controller::index(&db);
            // Process 'timestamps' and return a response as needed
        }
    };

    let timestamps_show_handler = |db: web::Data<DB>| {
        |req: HttpRequest| {
            let range = req.match_info().get("range").unwrap();
            let timestamps = timestamps_controller::show(&db, range);
            // Process 'timestamps' and return a response as needed
        }
    };

    let timestamps_create_handler = |db: web::Data<DB>| {
        |req: HttpRequest| {
            let result = timestamps_controller::create(&db, req);
            // Process 'result' and return a response as needed
        }
    };

    // Attach the handlers to the routes
    cfg.service(
        web::resource("api/locations")
            .route(web::get().to(locations_index_handler(&db))) // Handle GET request for /api/locations
            .route(web::post().to(locations_store_handler(&db))), // Handle POST request for /api/locations
    );

    cfg.service(
        web::resource("api/locations/{id}").route(web::get().to(locations_show_handler(&db))), // Handle GET request for /api/locations/{id}
    );

    cfg.service(
        web::resource("api/locations/{id}/timestamps")
            .route(web::get().to(locations_get_timestamps_handler(&db))), // Handle GET request for /api/locations/{id}/timestamps
    );

    cfg.service(
        web::resource("api/timestamps")
            .route(web::get().to(timestamps_index_handler(&db))) // Handle GET request for /api/timestamps
            .route(web::post().to(timestamps_create_handler(&db))), // Handle POST request for /api/timestamps
    );

    cfg.service(
        web::resource("api/timestamps/{range}").route(web::get().to(timestamps_show_handler(&db))), // Handle GET request for /api/timestamps/{range}
    );
}
