use diesel::PgConnection;
use r2d2;
use r2d2_diesel::ConnectionManager;
use std::env;
use std::ops::Deref;
use typemap::Key;

// Connection request guard type: a wrapper around an r2d2 pooled connection.
pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

// For the convenience of using an &DbConn as an &SqliteConnection.
impl Deref for DbConn {
  type Target = PgConnection;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

pub struct Pool;

impl Key for Pool {
  type Value = r2d2::Pool<ConnectionManager<PgConnection>>;
}

/// Initializes a database pool.
pub fn init_pool(
  db_url: Option<String>,
) -> r2d2::Pool<ConnectionManager<PgConnection>> {
  let database_url = match db_url {
    Some(url) => url,
    _ => env::var("DATABASE_URL").expect("DATABASE_URL must be defined"),
  };
  let manager = ConnectionManager::<PgConnection>::new(database_url);
  r2d2::Pool::builder()
    .build(manager)
    .expect("Failed to create pool.")
}
