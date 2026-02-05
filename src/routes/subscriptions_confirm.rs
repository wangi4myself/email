use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct Parmameters {
    pub subscription_token: String,
}

#[tracing::instrument(name = "Confirm a pending subscriber", skip(_parameters))]
pub async fn confirm_subscription(_parameters: web::Query<Parmameters>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
