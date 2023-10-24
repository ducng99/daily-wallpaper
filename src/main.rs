mod wallpaper_response;

use chrono::{Duration, Local};
use clap::Parser;
use std::path::Path;
use wallpaper_response::WallpaperResponse;

#[derive(Parser)]
struct Args {
    #[clap(long, required(true))]
    width: i32,
    #[clap(long, required(true))]
    height: i32,
}

const WALLPAPER_CACHE_PATH: &str = "/tmp/wallpaper-cache";

fn main() {
    let args = Args::parse();

    if args.width < 0 || args.height < 0 {
        println!("Width and height must be positive integers");
        return;
    }

    let current_time = Local::now();
    let current_date_formatted = current_time.format("%Y-%m-%d");
    let yesterday = current_time - Duration::days(1);
    let yesterday_formatted = yesterday.format("%Y-%m-%d");

    let today_wallpaper = get_today_wallpaper_cache(current_date_formatted.to_string());

    if today_wallpaper == None {
        remove_wallpaper(yesterday_formatted.to_string());
        if let Some(wallpaper_url) = get_today_wallpaper(args.width, args.height) {
            let wallpaper_path = Path::new(WALLPAPER_CACHE_PATH)
                .join(current_date_formatted.to_string())
                .with_extension("jpg");

            if let Ok(mut response_image) = reqwest::blocking::get(&wallpaper_url) {
                if std::fs::create_dir_all(WALLPAPER_CACHE_PATH).is_ok() {
                    if let Ok(mut file) = std::fs::File::create(wallpaper_path) {
                        let _ = response_image.copy_to(&mut file);
                    } else {
                        println!("Failed saving wallpaper image");
                    }
                } else {
                    println!("Failed to create wallpaper cache directory");
                }
            } else {
                println!("Failed downloading wallpaper image");
            }
        } else {
            println!("Failed to get today's wallpaper");
        }
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

fn get_today_wallpaper(width: i32, height: i32) -> Option<String> {
    let url = format!("https://bingwallpaper.microsoft.com/api/BWC/getHPImages?screenWidth={}&screenHeight={}&env=live", width, height);
    if let Ok(response) = reqwest::blocking::get(&url) {
        let response_text = response.text().unwrap_or_default();
        if !response_text.is_empty() {
            if let Ok(wallpaper_response) =
                serde_json::from_str::<WallpaperResponse>(&response_text)
            {
                if let Some(image_metadata) = wallpaper_response.images.get(0) {
                    return Some(image_metadata.url.to_owned());
                }
            }
        }
    }

    None
}
