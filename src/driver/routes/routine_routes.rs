use std::sync::Arc;
use axum::{Extension, Json};
use axum::extract::Path;
use bson::DateTime;
use bson::doc;
use bson::oid::ObjectId;
use mongodb::options::UpdateOptions;
use crate::application::dto::user_routine_dto::UserRoutineDTO;
use crate::application::use_case::routine;
use crate::application::use_case::routine::commands::sleep::SleepRequest;
use crate::application::use_case::task::queries::GetTasksError::DatabaseError;
use crate::driver::model::{Empty, HttpResponse};
use crate::driver::module::Modules;
use crate::infrastructure::database::entities::routine::UserRoutine;
use crate::shared::utils::date_time::now;

pub async fn user_sleep(
    Extension(modules): Extension<Arc<Modules>>,
    Json(request): Json<SleepRequest>,
) -> HttpResponse<Empty> {

    let options = UpdateOptions::builder()
        .upsert(true)
        .build();

    let collection = modules.driver.get_database().collection::<UserRoutine>("routine");
    let find_result = collection.find_one(doc! {"user_id": request.user_id.clone()}, None)
        .await
        .unwrap();
    if find_result.is_none(){
        return HttpResponse::fail_empty("找不到使用者資料");
    }
    collection.update_one(
        doc! {"user_id": request.user_id.clone()},
        doc! {
                "$set": {"sleeping": request.sleep}
            },
        None
    ).await;
    if find_result.unwrap().sleeping && !request.sleep{
        collection.update_one(
            doc! {"user_id": request.user_id.clone()},
            doc! {
                "$set": {"last_wakeup": DateTime::now()}
            },
            None
        ).await;
    }

    HttpResponse::success_empty("成功更新睡眠狀態")
}



pub async fn get_routine(
    Extension(modules): Extension<Arc<Modules>>,
    Path(user_id): Path<String>
) -> HttpResponse<UserRoutineDTO> {

    let collection = modules.driver.get_database().collection::<UserRoutine>("routine");
    let result = collection
        .find_one(doc! {"user_id": user_id.clone()}, None)
        .await;


    match result {
        Ok(routine) => {
            match routine {
                None => {
                    let routine = UserRoutine::new(user_id);
                    match collection.insert_one(routine.clone(), None).await {
                        Ok(r) => HttpResponse::success(UserRoutineDTO::from(routine),"success"),
                        Err(e) => HttpResponse::fail("fail", DatabaseError),
                    }
                }
                Some(r) => HttpResponse::success(UserRoutineDTO::from(r), "成功取得"),
            }
        }
        Err(e) => {
            println!("wwwwww{}", e);
            HttpResponse::fail("失敗", DatabaseError)
        },
    }
}


pub async fn user_eat(
    Extension(modules): Extension<Arc<Modules>>,
    Path(user_id): Path<String>
) -> HttpResponse<UserRoutine> {
    let options = UpdateOptions::builder()
        .upsert(true)
        .build();
    let collection = modules.driver.get_database().collection::<UserRoutine>("routine");

    let result = collection
        .update_one(doc! {"user_id": user_id.clone()},doc! {
            "$set": {"user_id": user_id.clone(),  "last_eaten": DateTime::now() },
            "$setOnInsert": { "last_wakeup": DateTime::now(), "sleeping": false},
        }, options ).await;


    match result {
        Ok(_) => HttpResponse::success_empty("成功更新吃飯狀態"),
        Err(e) => {
            println!("{}", e);
            HttpResponse::fail("更新狀態失敗", DatabaseError)
        },
    }
}