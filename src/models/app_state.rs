use crate::models::url_map::UrlMap;

#[derive(Clone)]
pub struct AppState {
    pub urls: UrlMap,
}


impl AppState {
    pub fn new() -> Self {
        Self {
            urls: UrlMap::new()
        }
    }

    pub fn get_info(&self) -> String {
        let url_count = self.urls.count();
        format!("Rustworld AppState\n  Urls: {url_count}")
    }
}