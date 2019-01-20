#[primary_key(role_id)]
#[derive(Debug)]
pub struct Role {
  pub role_id: i32,
  pub name: String,
}
