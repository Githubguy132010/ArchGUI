use std::sync::{Arc, Mutex};
use crate::installer::{InstallerConfig, DiskConfig, Partition};

#[derive(Default)]
pub struct ConfigStore {
    config: InstallerConfig,
    valid_pages: Vec<bool>,
}

impl ConfigStore {
    pub fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self {
            config: InstallerConfig::default(),
            valid_pages: vec![false; 4], // welkom, schijf, gebruiker, overzicht
        }))
    }

    pub fn set_disk_config(&mut self, device: String, use_encryption: bool, encryption_password: Option<String>) {
        self.config.disk_config = DiskConfig {
            device,
            use_encryption,
            encryption_password,
            partitions: vec![
                Partition {
                    mount_point: String::from("/boot/efi"),
                    size: String::from("512MiB"),
                    filesystem: String::from("fat32"),
                },
                Partition {
                    mount_point: String::from("/"),
                    size: String::from("100%"),
                    filesystem: String::from("ext4"),
                },
            ],
        };
    }

    pub fn set_user_config(&mut self, hostname: String, username: String, password: String) {
        self.config.hostname = hostname;
        self.config.username = username;
        self.config.password = password;
    }

    pub fn validate_page(&mut self, page_index: usize) -> bool {
        let valid = match page_index {
            0 => true, // Welkomstpagina is altijd geldig
            1 => !self.config.disk_config.device.is_empty(),
            2 => {
                !self.config.hostname.is_empty() 
                && !self.config.username.is_empty() 
                && !self.config.password.is_empty()
            },
            3 => self.valid_pages[1] && self.valid_pages[2],
            _ => false,
        };
        self.valid_pages[page_index] = valid;
        valid
    }
} 