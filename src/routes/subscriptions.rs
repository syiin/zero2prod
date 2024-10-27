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

impl TryFrom<FormData> for NewSubscriber {
    type Error = String;
    
    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(value.name)?;
        let email = SubscriberEmail::parse(value.email)?;
        Ok(Self { email, name })
    }
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
    let new_subscriber = match _form.0.try_into() {
        Ok(form) => form,
        Err(_) => return HttpResponse::BadRequest().finish(),
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
        INSERT INTO subscriptions (id, email, name, subscribed_at, status)
        VALUES ($1, $2, $3, $4, 'confirmed')
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