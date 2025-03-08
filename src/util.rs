use anyhow::{Context, Result};
use std::fs::File;
use std::io::copy;
use std::path::Path;
use std::path::PathBuf;

pub fn get_desktop_path_using_dirs() -> Option<PathBuf> {
    dirs::desktop_dir()
}

pub async fn download_file(url: &String, image_path: &String) -> Result<()> {
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
