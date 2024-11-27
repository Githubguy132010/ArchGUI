use anyhow::Result;
use serde::Deserialize;
use std::process::Command;
use tokio;

#[derive(Deserialize)]
struct Release {
    tag_name: String,
    assets: Vec<Asset>,
}

#[derive(Deserialize)]
struct Asset {
    browser_download_url: String,
    name: String,
}

pub struct Updater {
    current_version: String,
    github_repo: String,
}

impl Updater {
    pub fn new(current_version: String, github_repo: String) -> Self {
        Self {
            current_version,
            github_repo,
        }
    }

    pub async fn check_for_updates(&self) -> Result<Option<String>> {
        let client = reqwest::Client::new();
        let url = format!("https://api.github.com/repos/{}/releases/latest", self.github_repo);
        
        let release: Release = client
            .get(&url)
            .header("User-Agent", "Arch-Installer")
            .send()
            .await?
            .json()
            .await?;
            
        if release.tag_name != self.current_version {
            Ok(Some(release.tag_name))
        } else {
            Ok(None)
        }
    }

    pub async fn update(&self) -> Result<()> {
        // Download nieuwe versie
        let client = reqwest::Client::new();
        let url = format!("https://api.github.com/repos/{}/releases/latest", self.github_repo);
        
        let release: Release = client
            .get(&url)
            .header("User-Agent", "Arch-Installer")
            .send()
            .await?
            .json()
            .await?;
            
        for asset in release.assets {
            if asset.name == "arch-installer" {
                // Download binary
                let response = client
                    .get(&asset.browser_download_url)
                    .send()
                    .await?;
                    
                let bytes = response.bytes().await?;
                
                // Schrijf naar tijdelijk bestand
                let temp_path = "/tmp/arch-installer-new";
                std::fs::write(temp_path, bytes)?;
                
                // Maak uitvoerbaar
                Command::new("chmod")
                    .args(["+x", temp_path])
                    .output()?;
                    
                // Vervang oude binary
                Command::new("mv")
                    .args([temp_path, "/usr/bin/arch-installer"])
                    .output()?;
                    
                break;
            }
        }
        
        Ok(())
    }
} 