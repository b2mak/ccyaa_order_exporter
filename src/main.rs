mod download;
mod upload;

#[tokio::main]
async fn main() {
  let filename = "test";
  println!("Filename: {}", filename);
  let filepath = download::download_to_csv(&filename).await;

  let client = reqwest::Client::new();
  let _ = upload::create_or_update_file(
    &client,
    "folder id",
    &filepath,
  )
  .await;
}
