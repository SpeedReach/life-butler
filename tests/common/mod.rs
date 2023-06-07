use std::fmt::{Display, Formatter};
use error_stack::Context;
use life_butler::driver::module::Modules;
use tokio::sync::OnceCell;



static MODULES: OnceCell<Modules> = OnceCell::const_new();


pub async fn setup() -> &'static Modules{
    return MODULES.get_or_init( || async {
        let password = String::from("M9VWh2oRhEbUNjBx");
        Modules::new(password.as_str()).await
    }).await;
}

#[derive(Debug,Clone)]
pub struct TestError;

impl Display for TestError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "error while testing")
    }
}

impl Context for TestError{
    
}

