use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use chrono::Utc;
use uuid::Uuid;
use crate::domain::{SubscriberName, SubscriberEmail, NewSubscriber};

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
    let name = match SubscriberName::parse(_form.0.name) {
        Ok(name) => name,
        // Return early if the name is invalid, with a 400
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    let email = match SubscriberEmail::parse(_form.0.email) {
        Ok(email) => email,
        // Return early if the email is invalid, with a 400
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    let new_subscriber = NewSubscriber {
        email: email,
        name,
    };
    match insert_subscriber(&new_subscriber, &_pool).await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(_new_subscriber, _pool)
)]
pub async fn insert_subscriber(
    _new_subscriber: &NewSubscriber,
    _pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
            r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
                "#,
            Uuid::new_v4(),
            _new_subscriber.email.as_ref(),
            _new_subscriber.name.as_ref(),
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