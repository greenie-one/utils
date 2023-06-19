use lazy_static::lazy_static;
use tracing::info;

lazy_static! {
    pub static ref APP_ENV: String = std::env::var("APP_ENV").expect("APP_ENV should be defined");
    pub static ref FILE_VALIDATION_TIMEOUT: u64 =
        std::env::var("FILE_VALIDATION_TIMEOUT").expect("FILE_VALIDATION_TIMEOUT should be defined")
            .parse::<u64>()
            .expect("FILE_VALIDATION_TIMEOUT should be a number");
}

pub fn load_env() {
    info!("APP_ENV: {:?}", APP_ENV.as_str());
    dotenv::from_filename(format!("./.env.{}", APP_ENV.as_str())).unwrap();
}
