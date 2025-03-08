use anyhow::{Context, Result};
use std::fs::File;
use std::io::copy;
use std::path::Path;
use std::path::PathBuf;
use tokio;

#[cfg(target_os = "windows")]
use windows::{core::*, Win32::UI::Input::KeyboardAndMouse::*};

#[tokio::main]
async fn main() -> Result<()> {
    let desktop_path = get_desktop_path_using_dirs().unwrap();
    let target_name = String::from("downloaded_file.txt");
    let url = "https://w.wallhaven.cc/full/5g/wallhaven-5g22q5.png";
    let binding = String::from(desktop_path.join(target_name).to_str().unwrap());
    let image_path = binding.as_str();

    download_file(url, &image_path).await?;

    set_wallpaper(&image_path).await;
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

async fn download_file(url: &str, image_path: &str) -> Result<()> {
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
    let path = Path::new(image_path);
    let mut file = File::create(path).context(format!("Failed to create file: {}", image_path))?;

    // Copy the bytes to the file
    copy(&mut bytes.as_ref(), &mut file).context("Failed to write file to disk")?;

    println!("Successfully downloaded file to: {}", image_path);
    Ok(())
}

async fn set_wallpaper(image_path: &str) {
    wallpaper::set_from_path(image_path).expect("Something went wrong while setting wallpaper.");
    wallpaper::set_mode(wallpaper::Mode::Crop)
        .expect("Something went wrong while setting wallpaper mode.");
}
