use kankyo;
use lfgbot;

fn main() {
  // This will load the environment variables located at `./.env`, relative to
  // the CWD. See `./.env.example` for an example on how to structure this.
  kankyo::load().expect("Failed to load .env file");
  lfgbot::client_setup();
}
