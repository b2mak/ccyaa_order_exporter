use std::collections::HashMap;

pub async fn get_some_info() -> Result<(), gcp_auth::Error> {
  let token = get_auth_token().await?;
  let _ = gdrive_test_call(&token).await;

  println!("Did something happen?");
  return Ok(());
}

async fn gdrive_test_call(
  token: &gcp_auth::Token,
) -> Result<(), Box<dyn std::error::Error>> {
  let client = reqwest::Client::new();
  let response = client
    .get("https://www.googleapis.com/drive/v3/about?fields=*")
    .header("Authorization", format!("Bearer {}", token.as_str()))
    .send()
    .await?;

  let parsed = response.json::<HashMap<String, serde_json::Value>>().await;
  println!("Response {:?}", parsed);

  // match response.status() {
  //   reqwest::StatusCode::OK => {
  //     println!("Request worked {:?}", response);
  //     return Ok(());
  //   }
  //   reqwest::StatusCode::UNAUTHORIZED => {
  //     panic!("Invalid API token");
  //   }
  //   _ => {
  //     panic!("Unexpected status code");
  //   }
  // };
  return Ok(());
}

async fn get_auth_token() -> Result<gcp_auth::Token, gcp_auth::Error> {
  // `credentials_path` variable is the path for the credentials `.json` file.
  let credentials_path = std::path::PathBuf::from("PATH");
  let service_account =
    gcp_auth::CustomServiceAccount::from_file(credentials_path)?;
  let authentication_manager =
    gcp_auth::AuthenticationManager::from(service_account);
  let scopes = &["https://www.googleapis.com/auth/drive"];
  let token = authentication_manager.get_token(scopes).await?;

  return Ok(token);
}
