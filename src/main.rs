mod wallpaper_response;

use chrono::{Duration, Utc};
use clap::Parser;
use once_cell::sync::Lazy;
use std::{path::PathBuf, process::ExitCode};
use wallpaper_response::{Image, WallpaperResponse};

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
        path: PathBuf::from(&args.path),
    };

    if args.path.is_empty() {
        configs.path = std::env::temp_dir().join("wallpaper-cache");
    }

    configs
});

fn main() -> ExitCode {
    let today_time = Utc::now();
    let today_date_formatted = today_time.format("%Y%m%d").to_string();
    let yesterday_time = today_time - Duration::days(1);
    let yesterday_date_formatted = yesterday_time.format("%Y%m%d").to_string();

    let today_wallpaper = has_wallpaper_for_date(&today_date_formatted);

    if today_wallpaper {
        println!("Wallpaper already exists, skipping");
        ExitCode::SUCCESS
    } else {
        println!("Attempt to download wallpaper for today...");

        if let Some(wallpaper_images) = get_wallpapers(CONFIGS.width, CONFIGS.height) {
            if let Some(wallpaper_url) =
                get_wallpaper_for_date(&today_date_formatted, &wallpaper_images)
            {
                match save_wallpaper(&wallpaper_url, &today_date_formatted) {
                    Ok(_) => {
                        remove_wallpaper(&yesterday_date_formatted);
                        ExitCode::SUCCESS
                    }
                    Err(msg) => {
                        eprintln!("ERROR: {}", msg);
                        ExitCode::FAILURE
                    }
                }
            } else {
                println!("No wallpaper for today, getting latest wallpaper...");

                if let Some((date, url)) = get_wallpaper_latest(&wallpaper_images) {
                    if !has_wallpaper_for_date(&date) {
                        match save_wallpaper(&url, &date) {
                            Ok(_) => ExitCode::SUCCESS,
                            Err(msg) => {
                                eprintln!("ERROR: {}", msg);
                                ExitCode::FAILURE
                            }
                        }
                    } else {
                        println!("Latest wallpaper already exists, skipping");
                        ExitCode::SUCCESS
                    }
                } else {
                    eprintln!("Failed getting latest wallpaper.");
                    ExitCode::FAILURE
                }
            }
        } else {
            eprintln!("Failed getting wallpapers.");
            ExitCode::FAILURE
        }
    }
}

// Checks if a wallpaper exists for a specified date
fn has_wallpaper_for_date(date: &String) -> bool {
    let path = CONFIGS.path.join(date).with_extension("jpg");

    path.exists()
}

// Remove yesterday's wallpaper from cache if exists
fn remove_wallpaper(date: &String) {
    let path = CONFIGS.path.join(date).with_extension("jpg");

    if path.exists() {
        let _ = std::fs::remove_file(path);
    }
}

fn get_wallpapers(width: u32, height: u32) -> Option<Vec<Image>> {
    let url = format!("https://bingwallpaper.microsoft.com/api/BWC/getHPImages?screenWidth={}&screenHeight={}&env=live", width, height);
    if let Ok(response) = reqwest::blocking::get(&url) {
        let response_text = response.text().unwrap_or_default();
        if !response_text.is_empty() {
            if let Ok(wallpaper_response) =
                serde_json::from_str::<WallpaperResponse>(&response_text)
            {
                return Some(wallpaper_response.images);
            }
        }
    }

    None
}

fn get_wallpaper_for_date(date: &String, images: &Vec<Image>) -> Option<String> {
    for image_metadata in images {
        if image_metadata.startdate == date.to_owned() {
            return Some(image_metadata.url.to_string());
        }
    }

    None
}

fn get_wallpaper_latest(images: &Vec<Image>) -> Option<(String, String)> {
    if let Some(image_metadata) = images.first() {
        return Some((
            image_metadata.startdate.to_owned(),
            image_metadata.url.to_string(),
        ));
    }

    None
}

fn save_wallpaper(url: &String, date: &String) -> Result<(), &'static str> {
    let wallpaper_path = CONFIGS.path.join(date).with_extension("jpg");

    if let Ok(mut response_image) = reqwest::blocking::get(url) {
        if std::fs::create_dir_all(CONFIGS.path.clone()).is_ok() {
            if let Ok(mut file) = std::fs::File::create(&wallpaper_path) {
                if let Ok(_) = response_image.copy_to(&mut file) {
                    println!(
                        "Wallpaper saved to {}",
                        wallpaper_path.to_str().unwrap_or_default()
                    );

                    Ok(())
                } else {
                    Err("Failed saving wallpaper image")
                }
            } else {
                Err("Failed saving wallpaper image")
            }
        } else {
            Err("Failed to create wallpaper cache directory")
        }
    } else {
        Err("Failed downloading wallpaper image")
    }
}
