use actix_web::{
    post,
    web::{self, Json},
    HttpResponse, Scope,
};

use crate::{
    errors::ApiError,
    models::user::User,
    response::{respond_json, respond_ok},
    validate::validate_body,
    AppData,
};

pub fn auth_scope() -> Scope {
    Scope::new("/auth").service(register).service(login)
}

#[post("/register")]
async fn register(
    app: web::Data<AppData>,
    user: web::Json<User>,
) -> Result<HttpResponse, ApiError> {
    validate_body(&user)?;
    user.create_user(&app.db).await?;
    respond_ok()
}

#[post("/login")]
async fn login(app: web::Data<AppData>, user: web::Json<User>) -> Result<Json<String>, ApiError> {
    validate_body(&user)?;
    let res = user.login(&app.db, &app.secret).await?;
    respond_json(res)
}
