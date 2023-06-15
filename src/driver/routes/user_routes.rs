use crate::application::use_case::user::register_user::RegisterUserError;
use crate::application::use_case::user::user_login::UserLoginError;
use crate::driver::model::register_user::{RegisterUserRequest, RegisterUserResponse};
use crate::driver::model::user_login::{UserLoginRequest, UserLoginResponse};
use crate::driver::module::Modules;
use crate::infrastructure::database::entities::user::User;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use error_stack::Report;
use std::sync::Arc;
use crate::driver::model::HttpResponse;


#[utoipa::path(
post, path = "/user/register",
responses(
    (status = 200, description = "register a user",body = HttpResponse<RegisterUserRepsonse>)
))]
pub async fn register_user(
    Extension(modules): Extension<Arc<Modules>>,
    Json(request): Json<RegisterUserRequest>,
) -> HttpResponse<RegisterUserResponse> {
    let result = modules
        .register_user_use_case
        .register_user(request.email, request.password)
        .await;

    match result {
        Ok(user) => HttpResponse::success(RegisterUserResponse::from(user), "註冊成功"),
        Err(report) => HttpResponse::fail("註冊失敗",format!("{}", report.current_context())),
    }
}

#[utoipa::path(
post, path = "/user/login",
responses(
    (status = 200, description = "user login",body = HttpResponse<UserLoginResponse>)
))]
pub async fn user_login(
    Extension(modules): Extension<Arc<Modules>>,
    Json(request): Json<UserLoginRequest>,
) -> HttpResponse<UserLoginResponse> {
    let result = modules
        .user_login_use_case
        .login(request.email, request.password)
        .await;
    match result {
        Ok(user_id) => {
            HttpResponse::success(UserLoginResponse::from(user_id), "成功登入")
        }
        Err(report) => HttpResponse::fail("登入失敗",format!("{}", report.current_context())),
    }
}

