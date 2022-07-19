use crate::requests::all_game_data_response::AllGameDataResponse;
use crate::utils::requests::get;

use rocket::response::status;
use rocket::serde::json::Json;

#[get("/allgamedata")]
pub async fn all_game_data() -> Result<Json<AllGameDataResponse>, status::NotFound<()>> {
  match get::<AllGameDataResponse>("liveclientdata/allgamedata").await {
    Some(res) => Ok(Json(res)),
    None => Err(status::NotFound { 0: () }),
  }
}
