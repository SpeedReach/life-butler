use std::sync::Arc;
use axum::{Extension, Json};
use crate::application::use_case::event::commands::create_event::create_event_response::CreateEventResponse;
use crate::application::use_case::task::commands::create_task::create_task_request::CreateTaskRequest;
use crate::application::use_case::task::commands::create_task::create_task_response::CreateTaskResponse;
use crate::application::use_case::task::commands::update_task::update_task_request::UpdateTaskRequest;
use crate::application::use_case::task::commands::update_task::update_task_use_case::UpdateTaskUseCase;
use crate::application::use_case::task::queries::get_ongoing_tasks::get_ongoing_tasks_request::GetOnGoingTasksRequest;
use crate::application::use_case::task::queries::get_ongoing_tasks::get_ongoing_tasks_response::GetGoingTasksResponse;
use crate::application::use_case::task::queries::get_expired_tasks::get_expired_tasks_request::GetExpiredTasksRequest;
use crate::application::use_case::task::queries::get_expired_tasks::get_expired_tasks_response::GetExpiredTasksResponse;
use crate::driver::model::{Empty, HttpResponse};
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

pub async fn get_ongoing_tasks(
    Extension(modules): Extension<Arc<Modules>>,
    Json(request): Json<GetOnGoingTasksRequest>,
) -> HttpResponse<GetGoingTasksResponse> {
    let result = modules.get_done_tasks_use_case.get_tasks(request).await;
    match result {
        Ok(response) => HttpResponse::success(response, "成功取得進行中任務清單"),
        Err(error) => HttpResponse::fail("取得進行中任務清單失敗", error.current_context().clone())
    }
}

pub async fn get_expired_tasks(
    Extension(modules): Extension<Arc<Modules>>,
    Json(request): Json<GetExpiredTasksRequest>,
) -> HttpResponse<GetExpiredTasksResponse> {
    let result = modules.get_expired_tasks_use_case.get_tasks(request).await;
    match result {
        Ok(response) => HttpResponse::success(response, "成功取得過期的任務清單"),
        Err(error) => HttpResponse::fail("取得過期任務清單失敗", error.current_context().clone())
    }
}

pub async fn update_task(
    Extension(modules): Extension<Arc<Modules>>,
    Json(request): Json<UpdateTaskRequest>,
) -> HttpResponse<Empty> {
    let result = modules.update_task_status_use_case.update_task(request).await;
    match result { 
        Ok(res) => HttpResponse::success_empty("成功更新狀態"),
        Err(error) => HttpResponse::fail("更新任務失敗", error.current_context().clone())
    }
}