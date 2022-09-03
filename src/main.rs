mod download;
mod upload;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // download::download_to_csv();
  let _ = upload::get_some_info().await;

  return Ok(());
}
