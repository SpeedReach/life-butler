#![allow(dead_code)]
#![feature(async_fn_in_trait)]
#![allow(unused)]

use std::net::SocketAddr;
use axum::Router;

pub mod application;
pub mod infrastructure;
pub mod driver;



