

# ArchGUI ISO Builder

This repository contains the workflow to build an Arch Linux ISO with a KDE desktop environment and the project is called **ArchGUI**. The ISO is built automatically and released on GitHub daily.

## Features

- **Automated Daily Builds**: The ISO is built every day at midnight (UTC) and automatically uploaded to GitHub releases.
- **Arch Linux Base**: The ISO is built on the latest Arch Linux base, incorporating KDE as the desktop environment.
- **Installer Integration**: The ISO includes an installer script using `archinstall` that can be run via the desktop for easy setup.

## How the Workflow Works

The workflow is set up using GitHub Actions, with Docker to build the ISO inside an Arch Linux container. The process consists of the following steps:

1. **Checkout Repository**: Retrieves the latest content from the repository.
2. **Set up Arch Linux Container**: A Docker container simulating an Arch Linux environment is created.
3. **Build ISO**: The Arch Linux ISO is built using `mkarchiso`.
4. **Rename ISO**: The generated ISO is renamed to `ArchGUI.iso` for consistency.
5. **Upload ISO to GitHub**: The renamed ISO is uploaded as a release asset on GitHub.
6. **Clean Up**: The Docker container is stopped and removed after the build completes.

## How to Use

1. **Clone the repository**:

   ```bash
   git clone https://github.com/your-username/ArchGUI.git
   ```

2. **Run Workflow Automatically**: The workflow triggers on:
   - **Pushes** and **Pull Requests** to the `main` branch
   - **Scheduled daily builds** at midnight (UTC)

3. **Download the ISO**:
   - Visit the [releases page](https://github.com/your-username/ArchGUI/releases) to download the latest ISO.

## Configuration Details

The workflow utilizes a Docker-based Arch Linux container for a reliable and repeatable environment. The ISO build process includes:

- **Arch Linux setup**: The latest Arch Linux base is pulled using Docker's `archlinux:latest` image.
- **ISO generation**: The `mkarchiso` tool, part of the Arch Linux package set, is used to build the ISO.
- **Release process**: Each ISO is tagged with the current date and uploaded to the GitHub release page.

## GitHub Actions Workflow File

The workflow file responsible for this process is located in `.github/workflows/build-iso.yml`. Here is a relevant snippet:

```yaml
name: Build ISO

on:
  push:
    branches: [ "main" ]
  pull_request:
    branches: [ "main" ]
  schedule:
    # Runs every day at midnight
    - cron: '0 0 * * *'

jobs:
  build:
    runs-on: ubuntu-latest

    steps:
      - name: Checkout Repository
        uses: actions/checkout@v4

      - name: Set up Arch Linux Container
        run: |
          docker run --privileged --name arch-container -d -v ${{ github.workspace }}:/workdir archlinux:latest sleep infinity

      - name: Build ISO in Arch Container
        run: |
          docker exec arch-container bash -c "
          pacman -Syu --noconfirm &&
          pacman -S --noconfirm git archiso grub &&
          cd /workdir &&
          mkarchiso -v -w workdir/ -o out/ .
          "

      - name: Rename ISO to ArchGUI.iso
        run: |
          docker exec arch-container bash -c "
          iso_file=\$(ls /workdir/out/*.iso | head -n 1) &&
          mv \$iso_file /workdir/out/ArchGUI.iso
          "

      - name: Copy ISO to Host
        run: |
          docker cp arch-container:/workdir/out/ArchGUI.iso ${{ github.workspace }}/
      
      - name: Create GitHub Release
        uses: actions/create-release@v1.1.0
        env:
          GITHUB_TOKEN: ${{ secrets.PERSONAL_ACCESS_TOKEN }}
        with:
          tag_name: v${{ steps.date.outputs.date }}-release
          release_name: ${{ steps.date.outputs.date }}
          body: |
            This release contains the ArchGUI ISO built on ${{ steps.date.outputs.date }}.
          draft: false
          prerelease: false

      - name: Upload ISO to GitHub Release
        uses: actions/upload-release-asset@v1
        env:
          GITHUB_TOKEN: ${{ secrets.PERSONAL_ACCESS_TOKEN }}
        with:
          upload_url: ${{ steps.create_release.outputs.upload_url }}
          asset_path: ${{ github.workspace }}/ArchGUI.iso
          asset_name: ArchGUI.iso
          asset_content_type: application/octet-stream

      - name: Clean Up
        run: |
          docker stop arch-container
          docker rm arch-container
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
