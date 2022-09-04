use std::collections::HashMap;

pub async fn get_some_info() -> Result<(), gcp_auth::Error> {
  let token = get_auth_token().await?;
  // let _ = gdrive_files(&token).await;
  let _ = gdrive_upload_file(&token).await;

  println!("Did something happen?");
  return Ok(());
}

async fn get_body(boundry: &str) -> Result<String, Box<dyn std::error::Error>> {
  let file_content = std::fs::read_to_string("./export.csv")?;
  let parents = serde_json::json!({
      "parents": [
        "17V1roZrPEcwF_CpJqc9gtaOsfSi5erdb",
      ],
      "name": "test2",
  });

  let mut body = format!("--{}\n", boundry);
  body.push_str("Content-Type: application/json; charset=UTF-8\n");
  body.push_str("\n");
  body.push_str(&format!("{}\n", parents.to_string()));
  body.push_str(&format!("--{}\n", boundry));
  body.push_str("Content-Type: text/csv; charset=UTF-8\n");
  body.push_str("\n");
  body.push_str(&format!("{}\n", file_content));
  body.push_str(&format!("--{}--", boundry));

  return Ok(body);
}

async fn gdrive_upload_file(
  token: &gcp_auth::Token,
) -> Result<(), Box<dyn std::error::Error>> {
  let boundry = "xxxxxxxxxx";
  let body = get_body(boundry).await?;

  let client = reqwest::Client::new();
  let response = client
    .post(
      "https://www.googleapis.com/upload/drive/v3/files?uploadType=multipart",
    )
    .header("Authorization", format!("Bearer {}", token.as_str()))
    .header(
      "Content-Type",
      format!("multipart/related; boundary={}", boundry),
    )
    .header("Content-Length", body.len())
    .body(body)
    .send()
    .await?
    .json::<HashMap<String, serde_json::Value>>()
    .await;

  println!("Response {:?}", response);

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

async fn gdrive_files(
  token: &gcp_auth::Token,
) -> Result<(), Box<dyn std::error::Error>> {
  let client = reqwest::Client::new();
  let response = client
    .get("https://www.googleapis.com/drive/v3/files")
    .header("Authorization", format!("Bearer {}", token.as_str()))
    .send()
    .await?
    .json::<HashMap<String, serde_json::Value>>()
    .await;

  // This is mainly just for debugging so always output the response
  println!("Response {:?}", response);
  return Ok(());
}

async fn get_auth_token() -> Result<gcp_auth::Token, gcp_auth::Error> {
  // `credentials_path` variable is the path for the credentials `.json` file.
  let credentials_path = std::path::PathBuf::from("PATH");
  let service_account =
    gcp_auth::CustomServiceAccount::from_file(credentials_path)?;
  let authentication_manager =
    gcp_auth::AuthenticationManager::from(service_account);
  let scopes = ["https://www.googleapis.com/auth/drive"];
  let token = authentication_manager.get_token(&scopes).await?;

  return Ok(token);
}
