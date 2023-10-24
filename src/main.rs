mod wallpaper_response;

use chrono::{Duration, Local};
use std::path::Path;
use wallpaper_response::WallpaperResponse;

fn main() {
    let current_time = Local::now();
    let current_date_formatted = current_time.format("%Y-%m-%d");
    let yesterday = current_time - Duration::days(1);
    let yesterday_formatted = yesterday.format("%Y-%m-%d");

    let today_wallpaper = get_today_wallpaper_cache(current_date_formatted.to_string());

    if today_wallpaper == None {
        remove_wallpaper(yesterday_formatted.to_string());
    }
}

// Get today's wallpaper path from cache if exist. If not, return None.
fn get_today_wallpaper_cache(date: String) -> Option<String> {
    let path = Path::new("/tmp/wallpaper-cache").join(date);

    if path.exists() {
        let path_str = path.to_str().unwrap_or_default().to_owned();
        Some(path_str)
    } else {
        None
    }
}

// Remove yesterday's wallpaper from cache if exists
fn remove_wallpaper(date: String) {
    let path = Path::new("/tmp/wallpaper-cache").join(date);

    if path.exists() {
        let _ = std::fs::remove_file(path);
    }
}

fn get_today_wallpaper(width: i32, height: i32) -> String {
    let url = format!("https://bingwallpaper.microsoft.com/api/BWC/getHPImages?screenWidth={}&screenHeight={}&env=live", width, height);
    let request = reqwest::blocking::get(&url).unwrap_or(None);

    if request != None {
        let response_text = request.text().unwrap_or_default();
        if !response_text.is_empty() {
            let wallpaper_response: WallpaperResponse = serde_json::from_str(&response_text).unwrap_or(None);

            if wallpaper_response.is_some() {
                return wallpaper_response.unwrap().images[0].url;
            }
        }
    }

    String::from("")
}
