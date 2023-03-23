mod db;
mod errors;
mod extractors;
mod handlers;
mod models;
mod payments;
mod response;
mod validate;

use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use db::create_pool;
use dotenv::{self};
use env_logger::Env;
use sqlx::PgPool;

use handlers::auth::auth_scope;

use crate::handlers::links::links_scope;

#[derive(Clone)]
pub struct AppData {
    pub db: PgPool,
    pub secret: String,
    pub adyen_api_key: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let db_url = dotenv::var("DATABASE_URL").expect("PG_URL ENV VARIABLE IS NOT SET");
    let secret_key: String = dotenv::var("SECRET_KEY").expect("SECRET_KEY ENV VARIABLE IS NOT SET");
    let adyen_secret: String =
        dotenv::var("ADYEN_API_KEY").expect("ADYEN_API_KEY ENV VARIABLE IS NOT SET");
    let app_data = AppData {
        secret: secret_key,
        db: create_pool(&db_url).await,
        adyen_api_key: adyen_secret,
    };
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    sqlx::migrate!();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(Data::new(app_data.clone()))
            .service(auth_scope())
            .service(links_scope())
        // .service(
        //     web::scope("/api/v1")
        //         .service(auth_scope())
        // )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
