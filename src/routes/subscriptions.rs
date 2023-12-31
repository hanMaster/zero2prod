use actix_web::{web, HttpResponse};
use sqlx::{PgPool};
use sqlx::types::chrono::Utc;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}
pub async fn subscribe(form: web::Form<FormData>, conn: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query!(
        r#"
           INSERT INTO subscriptions (id, email, name, subscribed_at)
           VALUES($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(conn.get_ref())
    .await {
        Ok(_) => {
            log::info!("New subscriber details have been saved");
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            log::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
