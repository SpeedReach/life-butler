use std::sync::Arc;
use axum::{Extension, Json};
use crate::application::use_case::event::commands::create_event::create_event_response::CreateEventResponse;
use crate::application::use_case::task::commands::create_task::create_task_request::CreateTaskRequest;
use crate::application::use_case::task::commands::create_task::create_task_response::CreateTaskResponse;
use crate::driver::model::HttpResponse;
use crate::driver::module::Modules;

pub async fn create_task(
    Extension(modules): Extension<Arc<Modules>>,
    Json(request): Json<CreateTaskRequest>,
) -> HttpResponse<CreateTaskResponse> {
    let result = modules.create_task_use_case.create_task(request).await;
    match result {
        Ok(response) => HttpResponse::success(response, "成功創建任務"),
        Err(error) => HttpResponse::fail("創建任務失敗", error.current_context().clone())
    }
}