use actix_web::{
    delete, get, patch, post,
    web::{self, Json},
    HttpResponse, Scope,
};

use crate::{
    errors::ApiError,
    extractors::jwt::JWTToken,
    models::links::Link,
    response::{respond_json, respond_ok},
    validate::validate_body,
    AppData,
};

pub fn links_scope() -> Scope {
    Scope::new("/links")
        .service(retrieve_all)
        .service(create)
        .service(delete)
        .service(retrive_single)
        .service(update)
}

#[get("/")]
async fn retrieve_all(
    app: web::Data<AppData>,
    _jwt: JWTToken,
) -> Result<Json<Vec<Link>>, ApiError> {
    let links = Link::get_all(&app.db).await?;
    respond_json(links)
}

#[post("/")]
async fn create(
    app: web::Data<AppData>,
    _jwt: JWTToken,
    data: web::Json<Link>,
) -> Result<Json<Link>, ApiError> {
    validate_body(&data)?;
    let created_link = data.create(&app.adyen_api_key, &app.db).await?;
    respond_json(created_link)
}

#[delete("/{id}")]
async fn delete(
    app: web::Data<AppData>,
    _jwt: JWTToken,
    item: web::Path<(i32,)>,
) -> Result<HttpResponse, ApiError> {
    Link::delete(&app.db, item.0).await?;
    respond_ok()
}

#[get("/{id}")]
async fn retrive_single(
    app: web::Data<AppData>,
    _jwt: JWTToken,
    item: web::Path<(i32,)>,
) -> Result<Json<Link>, ApiError> {
    let link = Link::get(&app.db, item.0).await?;
    respond_json(link)
}

#[patch("/{id}")]
async fn update(
    app: web::Data<AppData>,
    _jwt: JWTToken,
    item: web::Path<(i32,)>,
    data: web::Json<Link>,
) -> Result<Json<Link>, ApiError> {
    validate_body(&data)?;
    let updated_link = data.update(&app.db, item.0).await?;
    respond_json(updated_link)
}
