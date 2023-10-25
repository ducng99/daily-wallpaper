# Daily wallpaper

Fetches new daily wallpaper from Bing Wallpaper and save as file.

Should be combined with tools such as `feh` to set wallpaper.

## Usage

This is a CLI-only tool. Ideally, it should be ran on computer starts, or your desktop manager starts (i3 config).

Example: creates `~/Pictures/wallpapers` directory and download image into that directory.

```bash
daily-wallpaper -p ~/Pictures/wallpapers
```

Example: get wallpaper for vertical screen

```bash
daily-wallpaper -p ~/Pictures/wallpapers --width 1080 --height 1920
```

Example: uses `feh` to set wallpaper

```bash
daily-wallpaper -p ~/Pictures/wallpapers && feh --bg-fill ~/Pictures/wallpapers/*
```

Help:

```bash
Usage: daily-wallpaper [OPTIONS]

Options:
      --width <WIDTH>    Ideal width [default: 1920]
      --height <HEIGHT>  Ideal height [default: 1080]
  -p, --path <PATH>      Path to save wallpaper image. Defaults to <tmp_dir>/wallpaper_cache [default: ]
  -h, --help             Print help
```
