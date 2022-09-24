mod download;
mod upload;

#[tokio::main]
async fn main() {
  // let _ = download::download_to_csv().await;
  // let _ = upload::get_some_info().await;
  let client = reqwest::Client::new();

  // println!("Files in folder {:?}", response);
}
