use anyhow::{Context, Result};
use std::fs::File;
use std::io::copy;
use std::path::Path;
use std::path::PathBuf;
use tokio;

#[cfg(target_os = "windows")]
use windows::{core::*, Win32::UI::Input::KeyboardAndMouse::*};

struct Wallpaper {
    url: String,
    image_path: String,
}
impl Wallpaper {
    fn new(url: String) -> Self {
        let desktop_path = get_desktop_path_using_dirs().unwrap();
        let target_name = String::from("downloaded_file.txt");
        let binding = String::from(desktop_path.join(target_name).to_str().unwrap());
        let image_path = binding.as_str();
        Wallpaper {
            url: String::from(url),
            image_path: String::from(image_path),
        }
    }

    async fn set_wallpaper(&mut self) {
        wallpaper::set_from_path(self.image_path.as_str())
            .expect("Something went wrong while setting wallpaper.");
        wallpaper::set_mode(wallpaper::Mode::Crop)
            .expect("Something went wrong while setting wallpaper mode.");
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut wallpaper = Wallpaper::new(String::from(
        "https://w.wallhaven.cc/full/5g/wallhaven-5g22q5.png",
    ));

    download_file(&wallpaper.url, &wallpaper.image_path).await?;

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

fn get_desktop_path_using_dirs() -> Option<PathBuf> {
    dirs::desktop_dir()
}

async fn download_file(url: &String, image_path: &String) -> Result<()> {
    // Create a client
    let client = reqwest::Client::new();

    // Send GET request and get the response
    let response = client
        .get(url)
        .send()
        .await
        .context("Failed to send request")?;

    // Ensure the request was successful
    if !response.status().is_success() {
        anyhow::bail!("Failed to download file: HTTP {}", response.status());
    }

    // Get the bytes
    let bytes = response
        .bytes()
        .await
        .context("Failed to get response bytes")?;

    // Create the output file
    let path = Path::new(image_path.as_str());
    let mut file = File::create(path).context(format!("Failed to create file: {}", image_path))?;

    // Copy the bytes to the file
    copy(&mut bytes.as_ref(), &mut file).context("Failed to write file to disk")?;

    println!("Successfully downloaded file to: {}", image_path);
    Ok(())
}
