mod gdrive;

pub async fn create_or_update_file(
  client: &reqwest::Client,
  folder_id: &str,
  filename: &str,
) -> () {
  let token = gdrive::get_auth_token().await.expect("Auth error");
  let response = gdrive::list_files_in_shared_folder(client, &token, folder_id)
    .await
    .expect("Get files error");
  // look at files
  let mut existing_file: Option<gdrive::structs::File> = None;
  for file in response.files.iter() {
    if file.name == filename {
      existing_file = Some(file.clone());
      break;
    }
  }

  match existing_file {
    Some(file) => gdrive::update_file(client, &token, &file.id)
      .await
      .expect("Update file blew up"),
    None => gdrive::create_file(client, &token, folder_id, filename)
      .await
      .expect("Create file blew up"),
  }

  return ();
}
