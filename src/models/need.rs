use crate::models::{role::Role, team::Team};

#[primary_key(need_id)]
#[derive(Debug, Associations)]
#[belongs_to(Team, Role)]
pub struct Need {
  pub need_id: i32,
  pub team_id: i32,
  pub role_id: i32,
}
