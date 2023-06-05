
use std::future::Future;
use life_butler::driver::module::Modules;
use once_cell::sync::Lazy;
use tokio::runtime::Runtime;
use tokio::sync::OnceCell;

mod user;


static MODULES: OnceCell<Modules> = OnceCell::const_new();


pub async fn setup() -> &'static Modules{
    return MODULES.get_or_init( || async {
        let password = std::env::args().nth(2).expect("please enter the password for mongodb");
        println!("{}",format!("password is {password:?}"));
        Modules::new(password.as_str()).await
    }).await;
}
