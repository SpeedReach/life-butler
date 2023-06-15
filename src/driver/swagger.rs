use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::driver::model::HttpResponse;
use crate::driver::model::register_user::{RegisterUserRequest, RegisterUserResponse};
use crate::driver::model::user_login::{UserLoginRequest, UserLoginResponse};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::driver::routes::user_routes::user_login,
        crate::driver::routes::user_routes::register_user,
    ),
    components(
        schemas(HttpResponse<UserLoginResponse>),
        schemas(HttpResponse<RegisterUserResponse>),
        schemas(HttpResponse<UserLoginRequest>),
        schemas(HttpResponse<RegisterUserRequest>),
        schemas(UserLoginResponse),
        schemas(RegisterUserResponse),
        schemas(UserLoginRequest),
        schemas(RegisterUserRequest),
    )
)]
pub struct ApiDoc;


fn test(){


}