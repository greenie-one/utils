use crate::env_config::APP_ENV;

#[derive(Clone)]
pub struct LeadsService {}
impl LeadsService {
    pub fn constuct_url(_container_name: String, file_name: String) -> String {
        let env = APP_ENV.as_str();
        let url = match env {
            "production" => format!("https://api.greenie.one/utils/leads/{}", file_name),
            _ => format!("https://dev-api.greenie.one/utils/leads/{}", file_name),
        };
        url
    }
}
