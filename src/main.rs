use anyhow::Result;
use tokio;
mod util;
use util::download_file;
mod wallpaper;
use wallpaper::Wallpaper;
#[cfg(target_os = "windows")]
use windows::{core::*, Win32::UI::Input::KeyboardAndMouse::*};

#[tokio::main]
async fn main() -> Result<()> {
    let mut wallpaper = Wallpaper::new(String::from(
        "https://w.wallhaven.cc/full/5g/wallhaven-5g22q5.png",
    ));

    download_file(&wallpaper.url, &wallpaper.image_path).await?;

    #[cfg(target_os = "windows")]
    change_keyboard_layout();

    wallpaper.set_wallpaper().await;
    std::process::exit(0)
}

#[cfg(target_os = "windows")]
fn change_keyboard_layout() {
    unsafe {
        let _ = LoadKeyboardLayoutA(
            PCSTR("00010439\0".as_ptr()), // Hindi
            KLF_ACTIVATE,
        );
    }
}
