mod download;
mod upload;

#[tokio::main]
async fn main() {
  // let _ = download::download_to_csv().await;

  let client = reqwest::Client::new();
  let _ = upload::create_or_update_file(
    &client,
    "folder id",
    "test3",
  ).await;
}
