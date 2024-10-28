use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use chrono::Utc;
use uuid::Uuid;
use crate::domain::{SubscriberName, SubscriberEmail, NewSubscriber};
use crate::email_client::EmailClient;
use crate::startup::ApplicationBaseUrl;

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
    skip(_form, _pool, _email_client, base_url),
    fields(
        subscriber_email = %_form.email,
        subscriber_name = %_form.name
    )
)]
pub async fn subscribe(
    _pool: web::Data<PgPool>,
    _form: web::Form<FormData>,
    _email_client: web::Data<EmailClient>,
    base_url: web::Data<ApplicationBaseUrl>,
) -> HttpResponse {
    let new_subscriber = match _form.0.try_into() {
        Ok(form) => form,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    if insert_subscriber(&_pool, &new_subscriber).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    if send_confirmation_email(
        &_email_client,
        new_subscriber,
        &base_url.0
    )
        .await
        .is_err()
    {
        return HttpResponse::InternalServerError().finish();
    }
    HttpResponse::Ok().finish()
}

#[tracing::instrument(
    name = "Send a confirmation email to a new subscriber",
    skip(_email_client, new_subscriber, base_url)
)]
async fn send_confirmation_email(
    _email_client: &EmailClient,
    new_subscriber: NewSubscriber,
    base_url: &str,
) -> Result<(), reqwest::Error>{
    let confirmation_link = format!("{}/subscriptions/confirm?subscription_token=mytoken", base_url);
    _email_client
        .send_email(
            new_subscriber.email,
            "Welcome!",
            &format!(
                "Welcome to our newsletter!<br />\
                Click <a href=\"{}\">here</a> to confirm your subscription.",
                confirmation_link
            ),
            &format!(
                "Welcome to our newsletter!\nVisit {} to confirm your subscription.",
                confirmation_link
            ),
        ).await
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(_new_subscriber, _pool)
)]
pub async fn insert_subscriber(
    _pool: &PgPool,
    _new_subscriber: &NewSubscriber,
    ) -> Result<(), sqlx::Error> {
    sqlx::query!(
            r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at, status)
        VALUES ($1, $2, $3, $4, 'pending_confirmation')
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