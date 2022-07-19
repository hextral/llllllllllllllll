use crate::config::CONFIG;

use serde::Deserialize;

pub async fn get<T>(url: &str) -> Option<T>
where
  T: for<'de> Deserialize<'de>,
{
  let config = CONFIG.read().unwrap().clone();

  let client = match reqwest::Client::builder()
    .danger_accept_invalid_certs(true)
    .build()
  {
    Ok(client) => client,
    Err(_) => {
      return None;
    }
  };

  let request = client.get(format!("{}/{}", config.llllll_url, url).as_str());

  let response = match request.send().await {
    Ok(res) => res,
    Err(_) => {
      return None;
    }
  };

  match response.json::<T>().await {
    Ok(res) => Some(res),
    Err(_) => None,
  }
}
