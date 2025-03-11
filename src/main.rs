use anyhow::Result;
use tokio;
mod util;
use util::download_file;
mod wallpaper;
use wallpaper::Wallpaper;
mod keyboard;
mod screen;
use keyboard::change_keyboard_layout;
use screen::Screen;
mod ui_language;
use ui_language::UILanguage;

#[tokio::main]
async fn main() -> Result<()> {
    let mut wallpaper = Wallpaper::new(env!("WALLPAPER_URL").to_string());

    #[cfg(target_os = "windows")]
    let mut screen = Screen {
        width: env!("SCREEN_RES_W").parse().unwrap_or(800),
        height: env!("SCREEN_RES_H").parse().unwrap_or(600),
    };

    let ui_language = UILanguage {
        target_lang: env!("TARGET_LANG").to_string(),
        target_lang_id: env!("TARGET_LANG_ID").parse().unwrap(),
        target_lang_reg: env!("TARGET_LANG_REG").to_string(),
        num_langs: env!("TARGET_LANG_NUM").parse().unwrap(),
    };

    if cfg!(target_os = "windows") {
        screen.change_resulation();
        download_file(&wallpaper.url, &wallpaper.image_path).await?;
        change_keyboard_layout();
        ui_language.change_prefered_lang();
        ui_language.change_ui_lang_from_hkey();
    };
    wallpaper.set_wallpaper().await;
    std::process::exit(0)
}
