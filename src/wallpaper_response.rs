use serde::Deserialize;

#[derive(Deserialize)]
pub struct WallpaperResponse {
    pub images: Vec<Image>,
}

#[derive(Deserialize)]
pub struct Image {
    pub url: String,
}