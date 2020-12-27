use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use std::sync::Arc;
use chrono::Utc;
use uuid::Uuid;
use std::ops::Deref;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>)
                       -> Result<HttpResponse, HttpResponse> {
    sqlx::query!(
    r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
    VALUES ($1, $2, $3, $4)
    "#,
    Uuid::new_v4(),
    form.email,
    form.name,
    Utc::now()
    )
        .execute(pool.get_ref())
        .await
        .map_err(|e| {
            eprintln!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    Ok(HttpResponse::Ok().finish())
}
