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
use std::{env, process};
use tray_item::TrayItem;
use utils::cors::CORS;

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
  let rocket_port: u16 = match env::var("LLLLLLLLLLLLLLLL_PORT") {
    Ok(port) => port.parse().unwrap(),
    Err(_) => 8923,
  };

  let mut tray = TrayItem::new("llllllllllllllll", "llllllllllllllll-icon").unwrap();

  tray
    .add_menu_item("Exit", move || {
      process::exit(0);
    })
    .unwrap();

  let config = rocket::Config {
    port: rocket_port,
    ..Default::default()
  };

  match rocket()
    .configure(config) //
    .attach(CORS) //
    .launch()
    .await
  {
    Ok(_) => (),
    Err(e) => {
      println!("{}", e);
      process::exit(1);
    }
  };
}
