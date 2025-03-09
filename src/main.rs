use anyhow::Result;
use tokio;
mod util;
use util::download_file;
mod wallpaper;
use wallpaper::Wallpaper;

#[cfg(target_os = "windows")]
mod keyboard;

#[cfg(target_os = "windows")]
mod screen;
#[cfg(target_os = "windows")]
use keyboard::change_keyboard_layout;

#[cfg(target_os = "windows")]
use screen::Screen;
#[tokio::main]
async fn main() -> Result<()> {
    let mut wallpaper = Wallpaper::new(String::from(
        "https://w.wallhaven.cc/full/5g/wallhaven-5g22q5.png",
    ));

    #[cfg(target_os = "windows")]
    let mut screen = Screen {
        width: 800,
        height: 600,
    };

    #[cfg(target_os = "windows")]
    screen.change_resulation();
    download_file(&wallpaper.url, &wallpaper.image_path).await?;

    #[cfg(target_os = "windows")]
    change_keyboard_layout();

    wallpaper.set_wallpaper().await;
    std::process::exit(0)
}
