use anyhow::Result;
use std::process::Command;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BlockDevice {
    pub name: String,
    pub size: u64,
    pub model: Option<String>,
    pub vendor: Option<String>,
}

pub fn get_available_disks() -> Result<Vec<BlockDevice>> {
    let output = Command::new("lsblk")
        .args(["--json", "--bytes", "--output", "NAME,SIZE,MODEL,VENDOR"])
        .output()?;
        
    let json = String::from_utf8(output.stdout)?;
    let devices: Vec<BlockDevice> = serde_json::from_str(&json)?;
    
    Ok(devices.into_iter()
        .filter(|dev| dev.name.starts_with("sd") || dev.name.starts_with("nvme"))
        .collect())
} 