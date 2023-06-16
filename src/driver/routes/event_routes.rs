use std::sync::Arc;
use axum::{Extension, Json};
use error_stack::Report;
use futures::future::err;
use crate::application::OperationErr;
use crate::application::use_case::event::commands::create_event::create_event_error::CreateEventError;
use crate::application::use_case::event::commands::create_event::create_event_request::CreateEventRequest;
use crate::application::use_case::event::commands::create_event::create_event_response::CreateEventResponse;
use crate::driver::model::HttpResponse;
use crate::driver::module::Modules;

pub async fn create_event(
    Extension(modules): Extension<Arc<Modules>>,
    Json(request): Json<CreateEventRequest>,
) -> HttpResponse<CreateEventResponse>{
    let result = modules.create_event_use_case.create_event(request).await;
    match result {
        Ok(data) => HttpResponse::success(data,"成功創建活動"),
        Err(error) => HttpResponse::fail("創建任務失敗",error.current_context().clone())
    }
}