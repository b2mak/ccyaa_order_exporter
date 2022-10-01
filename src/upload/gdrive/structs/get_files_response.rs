use std::vec::Vec;
use super::file::File;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetFilesResponse {
  pub kind: String,
  pub incomplete_search: bool,
  pub files: Vec<File>,
}
