mod gdrive;

pub async fn create_or_update_file(
  client: &reqwest::Client,
  auth_json: &str,
  folder_id: &str,
  filepath: &std::path::Path,
) -> () {
  let filename_oss = filepath.file_name();
  let filename_string = match filename_oss {
    Some(os_string) => {
      let string = os_string.to_str();
      match string {
        Some(string_type) => string_type,
        None => panic!("Unable to parse OS string to string"),
      }
    }
    None => {
      let str_filepath = filepath.to_str();
      match str_filepath {
        Some(filepath_str) => panic!("Filename in {} not found", filepath_str),
        None => {
          panic!("Filname not found and filepath not parsable to a string")
        }
      }
    }
  };

  let token = gdrive::get_auth_token(auth_json).await.expect("Auth error");
  let response = gdrive::list_files_in_shared_folder(client, &token, folder_id)
    .await
    .expect("Get files error");

  // look at files
  let mut existing_file: Option<gdrive::structs::File> = None;
  for file in response.files.iter() {
    if file.name == filename_string {
      existing_file = Some(file.clone());
      break;
    }
  }

  match existing_file {
    Some(file) => {
      println!("File with name {} found", filename_string);
      println!("Updating file with file ID: {}", &file.id);
      let response = gdrive::update_file(client, &token, &file.id, &filepath)
        .await
        .expect("Update file blew up");

      println!("File updated response:");
      println!("{:#?}", response);
    }
    None => {
      println!("File with name {} NOT found", filename_string);
      println!("Creating new file");
      let response = gdrive::create_file(
        client,
        &token,
        folder_id,
        filename_string,
        &filepath,
      )
      .await
      .expect("Create file blew up");

      println!("File created response:");
      println!("{:#?}", response);
    }
  }

  return ();
}
