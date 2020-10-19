#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket;

pub mod server;
pub mod models;
pub mod db_connection;
