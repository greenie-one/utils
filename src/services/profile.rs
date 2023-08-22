
#[derive(Clone)]
pub struct ProfileService {}
impl ProfileService {
    pub fn constuct_url(container_name: String, file_name: String) -> String {
        // Example URL https://fe981b19388e544fa86f77a.blob.core.windows.net/images/645a12409e423236816330b4.jpg
        let storage_account = std::env::var("STORAGE_ACCOUNT").expect("missing STORAGE_ACCOUNT");
        let url = format!(
            "https://{}.blob.core.windows.net/{}/{}",
            storage_account, container_name, file_name
        );
        url
    }
}
