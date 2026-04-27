# AGENTS.md

## Project

Single-binary Rust CLI that downloads Bing daily wallpaper and sets it as desktop background.

## Source layout

- `src/main.rs` — entry point, CLI args (`clap` derive), download/cache/set logic
- `src/wallpaper_response.rs` — Bing API response structs (`serde`)

## Commands

```
cargo build              # build
cargo run -- [opts]      # run (see README for CLI flags)
cargo fmt --all -- --check   # format check
cargo clippy --all-targets --all-features -- -D warnings   # lint (CI gate)
```

No tests exist in this repo.

## Lint config (Cargo.toml)

- `unsafe_code = "forbid"`
- Clippy: `all`, `pedantic`, `nursery`, `cargo` = warn
- Allowed: `module_name_repetitions`, `must_use_candidate`, `multiple_crate_versions`

## Rustfmt

- `style_edition = "2024"`, `use_field_init_shorthand = true`

## Toolchain

- Pinned to Rust 1.95 via `rust-toolchain.toml`
- Edition 2024

## CI (`.github/workflows/build.yml`)

- PRs to `main`: fmt check → clippy (deny warnings)
- Tags `v*.*.*`: cross-build for linux/win/macos, release via `git-cliff` + `softprops/action-gh-release`
- `cliff.toml` is referenced in CI but **not present in the repo** — release job will fail without it

## Key dependencies

- `reqwest` (blocking) — sync HTTP calls
- `clap` (derive) — CLI parsing
- `wallpaper` — sets desktop background (uses OS-native APIs; some DEs like i3 need `feh`)
- `anyhow`, `chrono`, `serde`, `serde_json`

## Notes

- Wallpaper cache defaults to `<tmp_dir>/wallpaper-cache` unless `-p` is passed
- Uses `LazyLock<Config>` for global CLI arg parsing
- Dates are formatted as `YYYYMMDD`; cached files are `<date>.jpg`
