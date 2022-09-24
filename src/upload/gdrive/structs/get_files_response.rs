use std::vec::Vec;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct File {
  pub id: String,
  pub kind: String,
  pub mime_type: String,
  pub name: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetFilesResponse {
  pub kind: String,
  pub incomplete_search: bool,
  pub files: Vec<File>,
}
