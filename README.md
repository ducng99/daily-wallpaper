# Daily wallpaper

Fetches new daily wallpaper from Bing Wallpaper and set it. It's that simple.

## Usage

This is a CLI-only tool. Ideally, it should be ran on computer starts, or your desktop manager starts (i3 config).

It uses [`wallpaper`](https://docs.rs/wallpaper/latest/wallpaper/) crate to set wallpaper, some desktop managers require a 3rd party tool to set background (eg. `feh` for i3).
To disable automatically setting the background, use `--noset` flag.

Example: creates `~/Pictures/wallpapers` directory and download image into that directory.

```bash
daily-wallpaper -p ~/Pictures/wallpapers
```

Example: get wallpaper for vertical screen

```bash
daily-wallpaper -p ~/Pictures/wallpapers --width 1080 --height 1920
```

Help:

```bash
Usage: daily-wallpaper [OPTIONS]

Options:
      --width <WIDTH>    Ideal width [default: 1920]
      --height <HEIGHT>  Ideal height [default: 1080]
  -p, --path <PATH>      Path to save wallpaper image. Defaults to <tmp_dir>/wallpaper_cache [default: ]
      --noset            Disable setting wallpaper
  -h, --help             Print help
```
