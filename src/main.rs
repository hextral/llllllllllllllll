#![windows_subsystem = "windows"]

#[macro_use]
extern crate rocket;

pub mod config;
pub mod requests;
pub mod routes;
pub mod utils;

use rocket::serde::json::Json;
use rocket::{Build, Rocket};
use routes::live_client_data;
use serde::Serialize;
use std::process;
use tray_item::TrayItem;

#[derive(Debug, Clone, Serialize)]
struct LlllllllllllllllStatus {
  running: bool,
}

#[get("/")]
fn index() -> Json<LlllllllllllllllStatus> {
  Json(LlllllllllllllllStatus { running: true })
}

fn rocket() -> Rocket<Build> {
  rocket::build()
    .mount("/", routes![index]) //
    .mount("/liveclientdata", routes![live_client_data::all_game_data]) //
}

#[tokio::main]
async fn main() {
  let mut tray = TrayItem::new("llllllllllllllll", "llllllllllllllll-icon").unwrap();

  tray
    .add_menu_item("Exit", move || {
      process::exit(0);
    })
    .unwrap();

  match rocket().launch().await {
    Ok(_) => (),
    Err(e) => {
      println!("{}", e);
      process::exit(1);
    }
  };
}
