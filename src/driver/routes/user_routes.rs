use std::sync::Arc;
use axum::http::StatusCode;
use axum::{Extension, Json};
use axum::response::{IntoResponse, Response};
use error_stack::Report;
use crate::application::use_case::user::register_user::RegisterUserError;
use crate::application::use_case::user::user_login::UserLoginError;
use crate::driver::models::register_user::{RegisterUseResponse, RegisterUserRequest};
use crate::driver::models::user_login::{UserLoginRequest, UserLoginResponse};
use crate::driver::module::Modules;
use crate::infrastructure::database::entities::user::User;

pub async fn register_user(
    Extension(modules): Extension<Arc<Modules>>,
    Json(request): Json<RegisterUserRequest>,

)-> Result<impl IntoResponse, RegisterUserError>{

    let result =modules.register_user_use_case.register_user(request.email, request.password).await;

    match result {
        Ok(user) => Ok(RegisterUseResponse(user)),
        Err(report) => Err(report.current_context().clone())
    }

}

pub async fn user_login(
    Extension(modules): Extension<Arc<Modules>>,
    Json(request): Json<UserLoginRequest>
) -> Result<impl IntoResponse,UserLoginError> {
    let result = modules.user_login_use_case.login(request.email,request.password).await;
    match result {
        Ok(user_id)=>Ok(UserLoginResponse::new(user_id)),
        Err(report)=>Err(report.current_context().clone())
    }
}
