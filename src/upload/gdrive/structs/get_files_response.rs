use super::file::File;
use std::vec::Vec;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetFilesResponse {
  pub kind: String,
  pub incomplete_search: bool,
  pub files: Vec<File>,
}
