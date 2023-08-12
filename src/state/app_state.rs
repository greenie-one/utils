use crate::services::doc_depot::DocDepotService;

#[derive(Clone)]
pub struct AppState {
    pub doc_depot_service: DocDepotService,
}