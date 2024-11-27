use anyhow::Result;
use std::process::Command;
use std::fs;

pub fn get_available_timezones() -> Result<Vec<String>> {
    let output = Command::new("timedatectl")
        .arg("list-timezones")
        .output()?;
    
    let timezones = String::from_utf8(output.stdout)?
        .lines()
        .map(String::from)
        .collect();
    
    Ok(timezones)
}

pub fn get_available_locales() -> Result<Vec<String>> {
    let content = fs::read_to_string("/usr/share/i18n/SUPPORTED")?;
    
    Ok(content
        .lines()
        .map(|line| line.split_whitespace().next().unwrap_or("").to_string())
        .filter(|locale| !locale.is_empty())
        .collect())
}

pub fn set_timezone(timezone: &str) -> Result<()> {
    Command::new("arch-chroot")
        .args(["/mnt", "ln", "-sf", &format!("/usr/share/zoneinfo/{}", timezone), "/etc/localtime"])
        .output()?;
    
    Command::new("arch-chroot")
        .args(["/mnt", "hwclock", "--systohc"])
        .output()?;
    
    Ok(())
}

pub fn configure_locale(locale: &str) -> Result<()> {
    // Voeg locale toe aan locale.gen
    fs::write("/mnt/etc/locale.gen", format!("{} UTF-8\n", locale))?;
    
    // Genereer locales
    Command::new("arch-chroot")
        .args(["/mnt", "locale-gen"])
        .output()?;
    
    // Stel standaard locale in
    fs::write("/mnt/etc/locale.conf", format!("LANG={}\n", locale))?;
    
    Ok(())
} 