use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct Parameters {
    subscription_token: String,
}

/// 确认订阅的路由处理器
/// GET /subscriptions/confirm?subscription_token=xxx
#[tracing::instrument(name = "Confirm a pending subscriber", skip(parameters, pool))]
pub async fn confirm_subscription(
    parameters: web::Query<Parameters>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    // 根据 token 查找订阅者 ID
    let subscriber_id =
        match get_subscriber_id_from_token(&pool, &parameters.subscription_token).await {
            Ok(Some(id)) => id,
            Ok(None) => return HttpResponse::Unauthorized().finish(),
            Err(_) => return HttpResponse::InternalServerError().finish(),
        };

    // 更新订阅状态为 confirmed
    if confirm_subscriber(&pool, subscriber_id).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}

/// 根据订阅令牌查找订阅者 ID
#[tracing::instrument(name = "Get subscriber_id from token", skip(pool, subscription_token))]
pub async fn get_subscriber_id_from_token(
    pool: &PgPool,
    subscription_token: &str,
) -> Result<Option<Uuid>, sqlx::Error> {
    let result = sqlx::query!(
        r#"SELECT subscriber_id FROM subscription_tokens WHERE subscription_token = $1"#,
        subscription_token
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(result.map(|r| r.subscriber_id))
}

/// 将订阅者状态更新为 confirmed
#[tracing::instrument(name = "Mark subscriber as confirmed", skip(pool, subscriber_id))]
pub async fn confirm_subscriber(pool: &PgPool, subscriber_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE subscriptions SET status = 'confirmed' WHERE id = $1"#,
        subscriber_id
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(())
}
