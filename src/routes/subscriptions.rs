use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use chrono::Utc;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(_form, _pool),
    fields(
        subscriber_email = %_form.email,
        subscriber_name = %_form.name
    )
)]
pub async fn subscribe(
    _pool: web::Data<PgPool>,
    _form: web::Form<FormData>,
) -> HttpResponse {
    match insert_subscriber(&_form, &_pool).await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(_form, _pool)
)]
pub async fn insert_subscriber(
    _form: &FormData, 
    _pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
            r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
                "#,
            Uuid::new_v4(),
            _form.email,
            _form.name,
            Utc::now()
        )
        .execute(_pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e  
        })?;
    Ok(())
}

