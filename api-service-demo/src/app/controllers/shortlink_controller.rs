use axum::Json;
use crate::app::models::dto;
use axum::extract::Extension;
use axum::response::IntoResponse;
use axum::http::StatusCode;
use crate::app::models::shortlink;
use sqlx::{Pool, MySql};

pub async fn create_shortlink(
    Json(req): Json<dto::CreateShortLinkReq>,
    Extension(pool): Extension<Pool<MySql>>,
) -> impl IntoResponse {
    println!("{:#?}", req);
    match shortlink::create_shortlink(&pool, &req.url).await {
        Ok(_) => {
            (StatusCode::OK, Json(dto::CreateUserResp {
                ok: true
            }))
        }
        Err(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(dto::CreateUserResp {
                ok: false
            }))
        }
    }
}

pub async fn delete_shortlink(
    Json(req): Json<dto::DeleteShortLinkReq>,
    Extension(pool): Extension<Pool<MySql>>,
) -> impl IntoResponse {
    println!("{:#?}", req);
    match shortlink::delete_shortlink(&pool, req.id).await {
        Ok(_) => {
            (StatusCode::OK, Json(dto::DeleteShortLinkResp {
                ok: true
            }))
        }
        Err(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(dto::DeleteShortLinkResp {
                ok: false
            }))
        }
    }
}