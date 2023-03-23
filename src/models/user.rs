use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Deserialize;
use sqlx::PgPool;
use validator::Validate;

use crate::errors::ApiError;

use crate::extractors::jwt::Claims;

#[derive(Validate, Deserialize)]
pub struct User {
    #[validate(length(
        min = 8,
        max = 64,
        message = "username must be between 8 and 64 characters long"
    ))]
    pub username: String,
    #[validate(custom(
        function = "crate::validate::validate_password",
        message = "Password should contain at least 1 digit, 1 special character, 1 uppercase character and be at least 8 characters long"
    ))]
    pub password: String,
}

struct UserWithId {
    id: i32,
    password: String,
}

impl User {
    pub async fn create_user(&self, db: &PgPool) -> Result<(), ApiError> {
        let mut salt = [0u8; 32];
        getrandom::getrandom(&mut salt).expect("COULD NOT GET RANDOM BYTES");
        let password_hash =
            argon2::hash_encoded(self.password.as_bytes(), &salt, &argon2::Config::default())
                .expect("ERROR WHILE UNWRAPPING PWD");

        sqlx::query!(
            "INSERT INTO users(username, password) VALUES($1, $2)",
            self.username,
            password_hash
        )
        .execute(db)
        .await?;
        Ok(())
    }

    pub async fn login(&self, db: &PgPool, secret: &String) -> Result<String, ApiError> {
        let user = sqlx::query_as!(
            UserWithId,
            "SELECT id, password FROM users WHERE username = $1",
            self.username
        )
        .fetch_one(db)
        .await?;

        let is_correct_pwd =
            argon2::verify_encoded(user.password.as_str(), self.password.as_bytes())
                .expect("CANNOT VERIFY PWD");

        if is_correct_pwd {
            Ok(create_token(user.id, secret))
        } else {
            Err(ApiError::Unauthorized(String::from("Bad credentenials")))
        }
    }
}

fn create_token(user_id: i32, secret: &String) -> String {
    let exp = (Utc::now() + Duration::days(365)).timestamp() as usize;
    let claims = Claims { id: user_id, exp };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap()
}
