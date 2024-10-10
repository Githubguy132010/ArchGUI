# Create the user 'ArchGUI-live' with password 'live'
useradd -m -G wheel -s /bin/bash ArchGUI-live
echo "ArchGUI-live:live" | chpasswd

# Enable SDDM
systemctl enable sddm

# Enable NetworkManager
systemctl enable NetworkManager