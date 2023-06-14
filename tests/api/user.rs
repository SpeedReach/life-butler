use std::collections::HashMap;
use crate::api::API_PATH;


#[tokio::test]
pub async fn register_api() {
    let mut map = HashMap::new();
    map.insert("email","brian0308@gmail.com");
    map.insert("password","123456");
    let client = reqwest::Client::new();
    let res = client.post(format!("{}/user/register", API_PATH))
        .json(&map).send().await
        .expect("err")
        .text().await.unwrap();
    println!("{}",res);
}

#[tokio::test]
pub async fn login_api() {
    let mut map = HashMap::new();
    map.insert("email","brian030128@gmail.com");
    map.insert("password","123456");
    let client = reqwest::Client::new();
    let res = client.post(format!("{}/user/login", API_PATH))
        .json(&map).send().await
        .expect("err")
        .text().await.unwrap();
    println!("{}",res);
}