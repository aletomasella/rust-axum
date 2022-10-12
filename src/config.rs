use std::env;
use dotenv::dotenv;

pub mod config {
pub struct Config {
  pub port : u16,
  pub host : String,
}

impl Config {
   fn new(port : u16, host : String) -> Config {
    Config {
      port,
      host,
    }
  }
}

pub fn read_config () -> Config {
  dotenv().ok();
  let port = env::var("PORT")
    .expect("PORT must be set")
    .parse()
    .expect("PORT must be a number");
  let host = env::var("HOST")
    .expect("HOST must be set");
  Config::new(port, host);
}
}



