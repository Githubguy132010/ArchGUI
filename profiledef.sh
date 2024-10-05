#!/usr/bin/env bash
# shellcheck disable=SC2034

# General ISO settings
iso_name="archlinux"
iso_label="ARCH_$(date --date="@${SOURCE_DATE_EPOCH:-$(date +%s)}" +%Y%m)"
iso_publisher="Arch Linux <https://archlinux.org>"
iso_application="Arch Linux Live/Rescue DVD"
iso_version="$(date --date="@${SOURCE_DATE_EPOCH:-$(date +%s)}" +%Y.%m.%d)"
install_dir="arch"
buildmodes=('iso')
bootmodes=('bios.syslinux.mbr' 'bios.syslinux.eltorito'
           'uefi-ia32.systemd-boot.esp' 'uefi-x64.systemd-boot.esp'
           'uefi-ia32.systemd-boot.eltorito' 'uefi-x64.systemd-boot.eltorito')
arch="x86_64"
pacman_conf="pacman.conf"
airootfs_image_type="squashfs"
airootfs_image_tool_options=('-comp' 'xz' '-Xbcj' 'x86' '-b' '1M' '-Xdict-size' '1M')
bootstrap_tarball_compression=('zstd' '-c' '-T0' '--auto-threads=logical' '--long' '-19')

# Define file permissions
file_permissions=(
  # Secure permission for /etc/shadow
  ["/etc/shadow"]="0:0:400"
  
  # Root directory permission
  ["/root"]="0:0:750"
  
  # Automated script (if you have one)
  ["/root/.automated_script.sh"]="0:0:755"
  
  # GnuPG directory for root (if used)
  ["/root/.gnupg"]="0:0:700"
  
  # Custom scripts permissions
  ["/usr/local/bin/choose-mirror"]="0:0:755"
  ["/usr/local/bin/Installation_guide"]="0:0:755"
  ["/usr/local/bin/livecd-sound"]="0:0:755"
  
  # Your custom setup script
  ["/root/custom-scripts/setup.sh"]="0:0:755"

  # Install Arch Linux desktop entry
  ["/etc/skel/Desktop/install-arch.desktop"]="0:0:755"

)

run_once=(
  "/root/custom-scripts/setup.sh"
)