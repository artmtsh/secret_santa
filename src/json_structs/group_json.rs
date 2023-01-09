#[derive(serde::Serialize, serde::Deserialize)]
pub struct UsernameGroupnameJson {
  pub caller_name: String,
  pub group_name: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SetAdminJson {
  pub caller_name: String,
  pub new_admin_name: String,
  pub group_name: String,
}