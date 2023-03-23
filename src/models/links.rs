use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use validator::Validate;

use crate::{errors::ApiError, payments::adyen::create_payment_link};

#[derive(Validate, Serialize, Deserialize)]
pub struct Link {
    #[serde(skip_deserializing)]
    pub id: i32,
    #[validate(length(
        min = 3,
        max = 64,
        message = "item name must be between 3 and 64 characters"
    ))]
    pub name: String,
    #[validate(range(min = 0, max = 1000, message = "price must be between 0 and 1000"))]
    pub price: i32,
    #[serde(skip_deserializing)]
    link: String,
}

impl Link {
    pub async fn get_all(db: &PgPool) -> Result<Vec<Link>, ApiError> {
        Ok(
            sqlx::query_as!(Link, "SELECT id, name, price, link FROM items ORDER BY id")
                .fetch_all(db)
                .await?,
        )
    }

    pub async fn create(&self, adyen_secret: &String, db: &PgPool) -> Result<Link, ApiError> {
        let payment_link = create_payment_link(adyen_secret, self).await?;

        let new_link = sqlx::query_as!(
            Link,
            "INSERT INTO items(name, price, link) VALUES($1, $2, $3) returning *",
            self.name,
            self.price,
            payment_link
        )
        .fetch_one(db)
        .await?;

        Ok(new_link)
    }

    pub async fn delete(db: &PgPool, link_id: i32) -> Result<(), ApiError> {
        sqlx::query!("DELETE FROM items where id = $1", link_id)
            .execute(db)
            .await?;
        Ok(())
    }

    pub async fn get(db: &PgPool, link_id: i32) -> Result<Link, ApiError> {
        let link = sqlx::query_as!(
            Link,
            "SELECT id, name, price, link FROM items where id = $1",
            link_id
        )
        .fetch_one(db)
        .await?;
        Ok(link)
    }

    pub async fn update(&self, db: &PgPool, link_id: i32) -> Result<Link, ApiError> {
        let link = sqlx::query_as!(
            Link,
            "UPDATE items SET price = $1, name = $2 where id = $3 RETURNING *",
            self.price,
            self.name,
            link_id
        )
        .fetch_one(db)
        .await?;
        Ok(link)
    }
}
