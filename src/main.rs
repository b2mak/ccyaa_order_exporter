mod download;
mod upload;

use std::collections::HashMap;
use substring::Substring;

#[tokio::main]
async fn main() {
  // Get secrets in memory
  println!("Input secrets json");
  let lines = std::io::stdin().lines();
  let mut json_str: String = "".to_owned();
  for line in lines {
    json_str.push_str(&line.unwrap());
  }
  let secrets: serde_json::Value = serde_json::from_str(&json_str).unwrap();
  // We need to clear the leading underscores
  let google_auth = serde_json::to_string(
    &(clear_leading_underscores(secrets["google_auth"].as_object().unwrap())
      .await),
  )
  .unwrap();

  let squarespace_auth = secrets["squarespace_api_token"].as_str().unwrap();

  let filename = "test";
  println!("Filename: {}", filename);
  let filepath = download::download_to_csv(&filename, &squarespace_auth).await;

  let client = reqwest::Client::new();
  upload::create_or_update_file(&client, &google_auth, "folder id", &filepath)
    .await;
}

async fn clear_leading_underscores(
  obj: &serde_json::Map<String, serde_json::Value>,
) -> HashMap<String, serde_json::Value> {
  let mut new_obj: HashMap<String, serde_json::Value> = HashMap::new();
  for (key, value) in obj {
    if key.starts_with("_") {
      new_obj.insert(key.substring(1, key.len()).to_owned(), value.clone());
    }
    new_obj.insert(key.clone(), value.clone());
  }
  return new_obj;
}
