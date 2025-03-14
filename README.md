# Taulixir

Taulixir is a sample project demonstrating how to integrate a Tauri desktop application with an Elixir-powered backend. The Elixir backend is packaged into a standalone binary using [Burrito](https://github.com/burrito-elixir/burrito).

> **Demo:** A Svelte-based counter UI provides buttons to increase, decrease, and refresh a counter. User actions are routed via HTTP requests to an Elixir (plug_cowboy) backend sidecar, and the updated counter value is returned to the UI for display.

---

## Table of Contents

- [Prerequisites](#prerequisites)
- [Setup and Installation](#setup-and-installation)
  - [Building the Elixir Binary](#building-the-elixir-binary)
  - [Integrating with Tauri](#integrating-with-tauri)
  - [Launching the Application](#launching-the-application)
- [Notes](#notes)

---

## Prerequisites

Ensure you have the following installed on your system:

- [Elixir](https://elixir-lang.org/install.html)
- [Tauri](https://v2.tauri.app/start/prerequisites/)
- [Rust](https://www.rust-lang.org/tools/install)
- [Zig [v0.13.0]](https://ziglang.org/download/) (required for Burrito)

---

## Setup and Installation

### Building the Elixir Binary

1. Navigate to the Elixir project directory [midway](/midway):

   ```bash
   cd ./midway
   ```

2. Build the release using Burrito. For example, for a Linux target in production mode, run:

   ```bash
   BURRITO_TARGET=linux MIX_ENV=prod mix release
   ```

   This command will create a single binary for the Elixir sidecar and output it in [midway/burrito_out](/midway/burrito_out).

### Integrating with Tauri

After the Elixir binary has been created:

1. Run the included script to copy and rename the binary to match Tauri’s sidecar naming conventions:

    ```bash
    ./copy_midway.sh
    ```

   This script moves the burrito binary into the [binaries](/binaries) folder and renames it, ensuring Tauri automatically includes it during the build process.
   For more details on the required naming conventions, please refer to the [Tauri sidecar documentation](https://v2.tauri.app/develop/sidecar/).

### Launching the Application

Navigate to the `app` subfolder and then start the Tauri application in development mode by executing:

```bash
cargo tauri dev
```

---

## Notes

- The Elixir sidecar runs on port **8080** and is accessible outside the Tauri environment.
- The binary packaging via Burrito simplifies deployment by bundling all dependencies. Please note that the current build has only been tested on Linux, so platform-specific configurations or adjustments may be required for macOS or Windows.
- Mobile support is completely untested.