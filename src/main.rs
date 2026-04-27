//! Downloads and sets the Bing daily wallpaper.
//!
//! This tool fetches wallpapers from the Bing wallpaper API, saves them to a
//! local cache, and sets them as the desktop background.

mod wallpaper_response;

use std::sync::LazyLock;

use anyhow::{Context, Result};
use chrono::{Duration, Utc};
use clap::Parser;
use std::path::PathBuf;
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
    #[clap(long, help = "Disable setting wallpaper")]
    noset: bool,
}

struct Config {
    width: u32,
    height: u32,
    path: PathBuf,
    disable_set_wallpaper: bool,
}

static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let args = Args::parse();

    let path = if args.path.is_empty() {
        std::env::temp_dir().join("wallpaper-cache")
    } else {
        PathBuf::from(&args.path)
    };

    Config {
        width: args.width,
        height: args.height,
        path,
        disable_set_wallpaper: args.noset,
    }
});

fn main() -> Result<()> {
    let today = Utc::now();
    let today_date = today.format("%Y%m%d").to_string();
    let yesterday_date = (today - Duration::days(1)).format("%Y%m%d").to_string();

    if has_wallpaper_for_date(&today_date) {
        set_wallpaper(&today_date);
        println!("Wallpaper already exists, skipping download");
        return Ok(());
    }

    println!("Download wallpaper for today...");

    let images = get_wallpapers(CONFIG.width, CONFIG.height)?;

    if let Some(url) = find_wallpaper_for_date(&today_date, &images) {
        save_wallpaper(&url, &today_date)?;
        remove_wallpaper(&yesterday_date);
        set_wallpaper(&today_date);
        return Ok(());
    }

    println!("No wallpaper for today, getting latest wallpaper...");

    let latest = images.first().context("No wallpapers available from API")?;
    let date = latest.startdate.to_owned();
    let url = latest.url.to_owned();

    if !has_wallpaper_for_date(&date) {
        save_wallpaper(&url, &date)?;
    } else {
        println!("Latest wallpaper already exists, skipping");
    }

    set_wallpaper(&date);
    Ok(())
}

fn has_wallpaper_for_date(date: &str) -> bool {
    CONFIG.path.join(date).with_extension("jpg").exists()
}

fn remove_wallpaper(date: &str) {
    let path = CONFIG.path.join(date).with_extension("jpg");
    if path.exists() {
        let _ = std::fs::remove_file(path);
    }
}

fn get_wallpapers(width: u32, height: u32) -> Result<Vec<wallpaper_response::Image>> {
    let url = format!(
        "https://bingwallpaper.microsoft.com/api/BWC/getHPImages?screenWidth={}&screenHeight={}&env=live",
        width, height
    );
    let response_text = reqwest::blocking::get(&url)
        .context("Failed to request wallpapers from API")?
        .text()
        .context("Failed to read API response body")?;

    let wallpaper_response: WallpaperResponse =
        serde_json::from_str(&response_text).context("Failed to parse wallpaper API response")?;

    Ok(wallpaper_response.images)
}

fn find_wallpaper_for_date<'a>(
    date: &str,
    images: &'a [wallpaper_response::Image],
) -> Option<&'a str> {
    images
        .iter()
        .find(|img| img.startdate == date)
        .map(|img| img.url.as_str())
}

fn save_wallpaper(url: &str, date: &str) -> Result<()> {
    let wallpaper_path = CONFIG.path.join(date).with_extension("jpg");

    let mut response = reqwest::blocking::get(url).context("Failed downloading wallpaper image")?;

    std::fs::create_dir_all(&CONFIG.path).context("Failed to create wallpaper cache directory")?;

    let mut file =
        std::fs::File::create(&wallpaper_path).context("Failed to create wallpaper image file")?;

    response
        .copy_to(&mut file)
        .context("Failed saving wallpaper image")?;

    println!(
        "Wallpaper saved to {}",
        wallpaper_path.to_str().unwrap_or_default()
    );

    Ok(())
}

fn set_wallpaper(date: &str) {
    if CONFIG.disable_set_wallpaper {
        return;
    }

    let path = CONFIG.path.join(date).with_extension("jpg");
    let _ = wallpaper::set_from_path(path.to_str().unwrap_or_default());
    let _ = wallpaper::set_mode(wallpaper::Mode::Span);
}
