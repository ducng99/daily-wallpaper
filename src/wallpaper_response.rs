use serde::Deserialize;

/// Response from the Bing wallpaper API.
#[derive(Debug, Deserialize)]
pub struct WallpaperResponse {
    pub images: Vec<Image>,
}

/// Metadata for a single wallpaper image.
#[derive(Debug, Deserialize)]
pub struct Image {
    /// Start date of the wallpaper in YYYYMMDD format.
    pub startdate: String,
    /// URL path to the wallpaper image.
    pub url: String,
}
