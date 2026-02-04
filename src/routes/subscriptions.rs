use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};
use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    pub name: String,
    pub email: String,
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
    skip(form, pool),
    fields(subscribe_email = %form.email, subscribe_name = %form.name)
)]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let new_subscriber = match form.0.try_into() {
        Ok(new_subscriber) => new_subscriber,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    match insert_subscriber(&pool, &new_subscriber).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            tracing::error!("Failed to add new subscriber: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    }
}

pub fn parse_subscriber_form(form: FormData) -> Result<NewSubscriber, String> {
    let name = SubscriberName::parse(form.name)?;
    let email = SubscriberEmail::parse(form.email)?;
    Ok(NewSubscriber { email, name })

    // 不使用 ? 操作符的等价写法：
    // let name = match SubscriberName::parse(form.name) {
    //     Ok(name) => name,
    //     Err(e) => return Err(e),
    // };
    // let email = match SubscriberEmail::parse(form.email) {
    //     Ok(email) => email,
    //     Err(e) => return Err(e),
    // };
    // Ok(NewSubscriber { email, name })
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(pool, new_subscriber)
)]
pub async fn insert_subscriber(
    pool: &PgPool,
    new_subscriber: &NewSubscriber,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"INSERT INTO subscriptions (id, email, name, subscribed_at, status) VALUES ($1, $2, $3, $4, 'confirmed')"#,
        Uuid::new_v4(),
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query:{:?}", e);
        e
    })?;
    Ok(())

    // 不使用 ? 操作符的等价写法：
    // match sqlx::query!(
    //     r#"INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)"#,
    //     Uuid::new_v4(),
    //     new_subscriber.email,
    //     new_subscriber.name.as_ref(),
    //     Utc::now()
    // )
    // .execute(pool)
    // .await
    // {
    //     Ok(_) => Ok(()),
    //     Err(e) => {
    //         tracing::error!("Failed to execute query:{:?}", e);
    //         Err(e)
    //     }
    // }
}
