mod wallpaper_response;

use chrono::{Duration, Local};
use clap::Parser;
use once_cell::sync::Lazy;
use std::{path::PathBuf, process::ExitCode};
use wallpaper_response::WallpaperResponse;

#[derive(Parser)]
struct Args {
    #[clap(long, default_value = "1920", help = "Ideal width")]
    width: u32,
    #[clap(long, default_value = "1080", help = "Ideal height")]
    height: u32,
    #[clap(
        short,
        long,
        default_value = "",
        help = "Path to save wallpaper image. Defaults to <tmp_dir>/wallpaper_cache"
    )]
    path: String,
}

struct Configs {
    width: u32,
    height: u32,
    path: PathBuf,
}

static CONFIGS: Lazy<Configs> = Lazy::new(|| {
    let args = Args::parse();

    let mut configs = Configs {
        width: args.width,
        height: args.height,
        path: PathBuf::from(args.path.clone()),
    };

    if args.path.is_empty() {
        configs.path = std::env::temp_dir().join("wallpaper-cache");
    }

    configs
});

fn main() -> ExitCode {
    let current_time = Local::now();
    let current_date_formatted = current_time.format("%Y-%m-%d");
    let yesterday = current_time - Duration::days(1);
    let yesterday_formatted = yesterday.format("%Y-%m-%d");

    let today_wallpaper = get_today_wallpaper_cache(current_date_formatted.to_string());

    if today_wallpaper == None {
        remove_wallpaper(yesterday_formatted.to_string());
        if let Some(wallpaper_url) = get_today_wallpaper(CONFIGS.width, CONFIGS.height) {
            let wallpaper_path = CONFIGS
                .path
                .join(current_date_formatted.to_string())
                .with_extension("jpg");

            if let Ok(mut response_image) = reqwest::blocking::get(&wallpaper_url) {
                if std::fs::create_dir_all(CONFIGS.path.clone()).is_ok() {
                    if let Ok(mut file) = std::fs::File::create(wallpaper_path.clone()) {
                        let _ = response_image.copy_to(&mut file);
                        println!(
                            "Wallpaper saved to {}",
                            wallpaper_path.to_str().unwrap_or_default()
                        );
                        return ExitCode::SUCCESS;
                    } else {
                        println!("Failed saving wallpaper image");
                        return ExitCode::FAILURE;
                    }
                } else {
                    println!("Failed to create wallpaper cache directory");
                    return ExitCode::FAILURE;
                }
            } else {
                println!("Failed downloading wallpaper image");
                return ExitCode::FAILURE;
            }
        } else {
            println!("Failed to get today's wallpaper");
            return ExitCode::FAILURE;
        }
    }

    ExitCode::SUCCESS
}

// Get today's wallpaper path from cache if exist. If not, return None.
fn get_today_wallpaper_cache(date: String) -> Option<String> {
    let path = CONFIGS.path.join(date);

    if path.exists() {
        let path_str = path.to_str().unwrap_or_default().to_owned();
        Some(path_str)
    } else {
        None
    }
}

// Remove yesterday's wallpaper from cache if exists
fn remove_wallpaper(date: String) {
    let path = CONFIGS.path.join(date);

    if path.exists() {
        let _ = std::fs::remove_file(path);
    }
}

fn get_today_wallpaper(width: u32, height: u32) -> Option<String> {
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
