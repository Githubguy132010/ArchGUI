use super::InstallerStep;
use anyhow::Result;
use std::fs;

pub struct NetworkConfigurer;

impl InstallerStep for NetworkConfigurer {
    fn execute(&self, config: &InstallerConfig) -> Result<()> {
        // Configureer hostname
        fs::write("/mnt/etc/hostname", &config.hostname)?;
        
        // Configureer hosts bestand
        let hosts = format!(
            "127.0.0.1    localhost\n\
             ::1          localhost\n\
             127.0.1.1    {}.localdomain    {}\n",
            config.hostname, config.hostname
        );
        fs::write("/mnt/etc/hosts", hosts)?;
        
        // Installeer en enable NetworkManager
        Command::new("arch-chroot")
            .args(["/mnt", "pacman", "-S", "--noconfirm", "networkmanager"])
            .output()?;
            
        Command::new("arch-chroot")
            .args(["/mnt", "systemctl", "enable", "NetworkManager"])
            .output()?;
            
        Ok(())
    }

    fn undo(&self) -> Result<()> {
        // Verwijder netwerk configuratie
        fs::remove_file("/mnt/etc/hostname")?;
        fs::remove_file("/mnt/etc/hosts")?;
        Ok(())
    }
} 