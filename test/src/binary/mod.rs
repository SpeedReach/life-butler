
use std::future::Future;
use life_butler::driver::module::Modules;
use once_cell::sync::Lazy;
use tokio::runtime::Runtime;
use tokio::sync::OnceCell;

mod user;


static MODULES: OnceCell<Modules> = OnceCell::const_new();


pub async fn setup() -> &'static Modules{
    return MODULES.get_or_init( || async {
        let password = String::from("c177267w");
        Modules::new(password.as_str()).await
    }).await;
}
