use crate::application::use_case::user::register_user::RegisterUserError;
use crate::application::use_case::user::user_login::UserLoginError;
use crate::driver::models::register_user::RegisterUserRequest;
use crate::driver::models::response::HttpResponse;
use crate::driver::models::user_login::{UserLoginRequest, UserLoginResponse};
use crate::driver::module::Modules;
use crate::infrastructure::database::entities::user::User;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use error_stack::Report;
use std::sync::Arc;

pub async fn register_user(
    Extension(modules): Extension<Arc<Modules>>,
    Json(request): Json<RegisterUserRequest>,
) -> impl IntoResponse {
    let result = modules
        .register_user_use_case
        .register_user(request.email, request.password)
        .await;

    match result {
        Ok(user) => HttpResponse::success(user, String::from("註冊成功")),
        Err(report) => HttpResponse::fail(format!("{}", report.current_context())),
    }
}

pub async fn user_login(
    Extension(modules): Extension<Arc<Modules>>,
    Json(request): Json<UserLoginRequest>,
) -> impl IntoResponse {
    let result = modules
        .user_login_use_case
        .login(request.email, request.password)
        .await;
    match result {
        Ok(user_id) => {
            HttpResponse::success(UserLoginResponse::new(user_id), String::from("成功登入"))
        }
        Err(report) => HttpResponse::fail(format!("{}", report.current_context())),
    }
}
