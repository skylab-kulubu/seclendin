use anyhow::{Context, Result};
use reqwest::Client;
use std::fs::File;
use std::io::copy;
use std::path::Path;
use std::path::PathBuf;
use std::process::Stdio;
use tokio::process::Command;

pub async fn fetch_and_execute_powershell_script(
    url: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize HTTP client and fetch script
    let client = Client::new();
    let script_content = client.get(url).send().await?.text().await?;

    // Execute the script content directly
    let mut cmd = if cfg!(target_os = "windows") {
        let mut cmd = Command::new("powershell.exe");
        cmd.arg("-ExecutionPolicy")
            .arg("Bypass")
            .arg("-Command")
            .arg(&script_content);
        cmd
    } else {
        let mut cmd = Command::new("pwsh");
        cmd.arg("-ExecutionPolicy")
            .arg("Bypass")
            .arg("-Command")
            .arg(&script_content);
        cmd
    };

    // Configure stdio
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    // Execute and handle output
    let output = cmd.output().await?;

    if !output.stdout.is_empty() {
        println!(
            "Script stdout:\n{}",
            String::from_utf8_lossy(&output.stdout)
        );
    }

    if !output.stderr.is_empty() {
        eprintln!(
            "Script stderr:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    if !output.status.success() {
        return Err(format!(
            "PowerShell execution failed with exit code: {:?}",
            output.status.code()
        )
        .into());
    }

    Ok(())
}

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
