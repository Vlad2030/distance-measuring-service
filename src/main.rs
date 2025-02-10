mod api;
mod core;
mod models;
mod utils;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env("LOG_LEVEL");

    let app = core::config::App::new();

    log::info!("{} {}", app.title, app.version);
    log::info!(
        "Service starting.. http://{}:{}",
        app.host,
        app.port,
    );
    ntex::web::HttpServer::new(move || {
        ntex::web::App::new()
            .service((
                api::handlers::health::healthcheck,
                api::handlers::distance::get_units,
                api::handlers::distance::get_distance,
            ))
            .wrap(ntex::web::middleware::Logger::default())
    })
    .bind(app.bind())?
    .workers(app.workers)
    .run()
    .await
}
