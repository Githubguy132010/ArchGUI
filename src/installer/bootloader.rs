use super::InstallerStep;
use anyhow::{Result, Context};
use std::process::Command;

pub struct BootloaderInstaller;

impl InstallerStep for BootloaderInstaller {
    fn execute(&self, config: &InstallerConfig) -> Result<()> {
        // Installeer GRUB packages
        Command::new("arch-chroot")
            .args(["/mnt", "pacman", "-S", "--noconfirm", "grub", "efibootmgr"])
            .output()
            .context("Kon GRUB niet installeren")?;
        
        // Configureer GRUB voor LUKS indien nodig
        if config.disk_config.use_encryption {
            let mut cmdline = String::new();
            cmdline.push_str("GRUB_CMDLINE_LINUX=\"cryptdevice=UUID=");
            
            // Haal UUID op van de LUKS partitie
            let uuid = Command::new("blkid")
                .arg("-s")
                .arg("UUID")
                .arg("-o")
                .arg("value")
                .arg("/dev/sda2") // Aanpassen aan juiste partitie
                .output()?;
            
            cmdline.push_str(&String::from_utf8(uuid.stdout)?);
            cmdline.push_str(":cryptroot root=/dev/mapper/cryptroot\"");
            
            std::fs::write("/mnt/etc/default/grub", cmdline)?;
        }
        
        // Installeer GRUB
        Command::new("arch-chroot")
            .args(["/mnt", "grub-install", "--target=x86_64-efi", "--efi-directory=/boot/efi", "--bootloader-id=GRUB"])
            .output()
            .context("Kon GRUB niet installeren")?;
            
        // Genereer config
        Command::new("arch-chroot")
            .args(["/mnt", "grub-mkconfig", "-o", "/boot/grub/grub.cfg"])
            .output()
            .context("Kon GRUB config niet genereren")?;
            
        Ok(())
    }

    fn undo(&self) -> Result<()> {
        // Verwijder GRUB bestanden
        std::fs::remove_dir_all("/mnt/boot/grub")?;
        Ok(())
    }
} 