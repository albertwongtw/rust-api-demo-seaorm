use actix_web::{dev::Response, web::Json};
use sea_orm::DatabaseConnection;

#[derive(serde::Deserialize, Debug, serde::Serialize)]
pub struct LoginUser {
    username: String,
    password: String,
}

#[derive(serde::Deserialize, Debug, serde::Serialize)]
pub struct ResponseData {
    success: bool,
    message: Option<String>,
    data: Option<serde_json::Value>,
}

#[tracing::instrument(name = "Logging a user in", skip(pool, user), fields(username = %user.username))]
#[actix_web::post("/login")]
async fn login(
    pool: actix_web::web::Data<DatabaseConnection>,
    user: actix_web::web::Json<LoginUser>,
) -> actix_web::HttpResponse {
    let username = &user.username;
    let password = user.password.as_bytes();
    match service::Query::verify_user(&pool, username, password)
        .await {
            Ok(exist) => {
                actix_web::HttpResponse::Ok().json(Json(ResponseData {
                    success: exist,
                    message: None,
                    data: None
                }))
            },
            Err(e) => {
                panic!("{:#?}", e)
            }
    }
}

pub fn routes_config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        actix_web::web::scope("/account")
            .service(login)
    );
}