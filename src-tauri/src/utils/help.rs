use lazy_static::lazy_static;
use std::env;

#[derive(PartialEq)]
pub enum AppEnv {
    Dev,
    Prod,
}

lazy_static! {
    /// Read from the environment variable NODE_ENV.
    /// The default value is AppEnv::Dev("development").
    pub static ref NODE_ENV: AppEnv = {
        match env::var("NODE_ENV") {
          Ok(env) if env == "production" => AppEnv::Prod,
          _ => AppEnv::Dev
        }
    };

    /// Is it a "production" environment
    pub static ref IS_PROD :bool = {
      *NODE_ENV == AppEnv::Prod
    };
}
