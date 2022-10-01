#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct File {
  pub id: String,
  pub kind: String,
  pub mime_type: String,
  pub name: String,
}
