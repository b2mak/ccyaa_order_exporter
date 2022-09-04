mod download;
mod upload;

#[tokio::main]
async fn main() {
  let _ = download::download_to_csv().await;
  let _ = upload::get_some_info().await;
}
