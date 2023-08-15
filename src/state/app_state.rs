use crate::services::{doc_depot::DocDepotService, admin::AdminService};

#[derive(Clone)]
pub struct DocDepotState {
    pub service: DocDepotService,
}

#[derive(Clone)]
pub struct AdminState {
    pub service: AdminService,
}

impl AdminState {
    pub async fn new() -> Self {
        Self {
            service: AdminService::new().await,
        }
    }
}
    