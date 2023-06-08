use std::fmt::{Display, Formatter};
use error_stack::Context;
use life_butler::driver::module::Modules;
use tokio::sync::OnceCell;



static MODULES: OnceCell<Modules> = OnceCell::const_new();


pub async fn setup() -> &'static Modules{

    return MODULES.get_or_init( || async {
        let arg = std::env::args().nth(2).expect("enter password: cargo test test <PASSWORD>");
        let password = &arg.clone();
        Modules::new(password).await
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

