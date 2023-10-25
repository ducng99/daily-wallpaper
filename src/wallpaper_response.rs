use serde::Deserialize;

#[derive(Deserialize)]
pub struct WallpaperResponse {
    pub images: Vec<Image>,
}

#[derive(Deserialize)]
pub struct Image {
    pub startdate: String,
    pub url: String,
}
