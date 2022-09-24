pub mod structs;

pub async fn get_auth_token() -> Result<gcp_auth::Token, gcp_auth::Error> {
  // `credentials_path` variable is the path for the credentials `.json` file.
  let credentials_path = std::path::PathBuf::from(
    "/Users/bmak/Code/secrets/ccyaa-order-exporter-1f2f0a6f85fe.json",
  );
  let service_account =
    gcp_auth::CustomServiceAccount::from_file(credentials_path)?;
  let authentication_manager =
    gcp_auth::AuthenticationManager::from(service_account);
  let scopes = ["https://www.googleapis.com/auth/drive"];
  let token = authentication_manager.get_token(&scopes).await?;

  return Ok(token);
}

pub async fn list_files_in_shared_folder(
  client: &reqwest::Client,
  token: &gcp_auth::Token,
  folder_id: &str,
) -> Result<structs::GetFilesResponse, reqwest::Error> {
  let response = client
    .get("https://www.googleapis.com/drive/v3/files")
    .header(
      reqwest::header::AUTHORIZATION,
      format!("Bearer {}", token.as_str()),
    )
    .header(reqwest::header::USER_AGENT, "CCYAA Order Exporter")
    .header(reqwest::header::CONTENT_TYPE, "application/json")
    .query(&[("q", format!("'{}' in parents", folder_id))])
    .send()
    .await?
    .json::<structs::GetFilesResponse>()
    .await?;

  return Ok(response);
}

pub async fn update_file(
  client: &reqwest::Client,
  token: &gcp_auth::Token,
  file_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
  let boundry = "xxxxxxxxxx";
  // We aren't modifying any metadata, just updating the content
  let metadata = serde_json::json!({});
  let body = get_body(boundry, &metadata).await?;

  make_request(
    format!(
      "https://www.googleapis.com/upload/drive/v2/files/{}?uploadType=multipart",
      file_id,
    ),
    client,
    token,
    body,
    boundry,
  ).await?;

  return Ok(());
}

pub async fn create_file(
  client: &reqwest::Client,
  token: &gcp_auth::Token,
  folder_id: &str,
  filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
  let boundry = "xxxxxxxxxx";
  let metadata = serde_json::json!({
      "parents": [
        folder_id,
      ],
    "name": filename,
  });
  let body = get_body(boundry, &metadata).await?;

  make_request(
    "https://www.googleapis.com/upload/drive/v3/files?uploadType=multipart",
    client,
    token,
    body,
    boundry,
  )
  .await?;

  // If there was an issue make_request would have blown up
  return Ok(());
}

async fn make_request(
  uri: &str,
  client: &reqwest::Client,
  token: &gcp_auth::Token,
  body: String,
  boundry: &str,
) -> Result<(), Box<dyn std::error::Error>> {
  let response = client
    .post(uri)
    .header("Authorization", format!("Bearer {}", token.as_str()))
    .header(
      "Content-Type",
      format!("multipart/related; boundary={}", boundry),
    )
    .header("Content-Length", body.len())
    .body(body)
    .send()
    .await?;

  match response.status() {
    reqwest::StatusCode::OK => {
      println!("File created");
      return Ok(());
    }
    reqwest::StatusCode::UNAUTHORIZED => {
      panic!("Invalid API token");
    }
    _ => {
      panic!("File creation call failed");
    }
  };
}

async fn get_body(
  boundry: &str,
  metadata: &serde_json::Value,
) -> Result<String, Box<dyn std::error::Error>> {
  let file_content = std::fs::read_to_string("./export.csv")?;

  let mut body = format!("--{}\n", boundry);
  body.push_str("Content-Type: application/json; charset=UTF-8\n");
  body.push_str("\n");
  body.push_str(&format!("{}\n", metadata.to_string()));
  body.push_str(&format!("--{}\n", boundry));
  body.push_str("Content-Type: text/csv; charset=UTF-8\n");
  body.push_str("\n");
  body.push_str(&format!("{}\n", file_content));
  body.push_str(&format!("--{}--", boundry));

  return Ok(body);
}
