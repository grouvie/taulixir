# Taulixir

Taulixir is a sample project demonstrating how to integrate a Tauri desktop application with an Elixir-powered backend.

> **Demo:** A Svelte-based counter UI provides buttons to increase, decrease, and refresh a counter. User actions are routed via HTTP requests to an Elixir (plug_cowboy) backend sidecar, which then returns the updated counter value to the UI for display.

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

Ensure the following are installed on your system:

- [Elixir](https://elixir-lang.org/install.html)
- [Tauri](https://v2.tauri.app/start/prerequisites/)
- [Rust](https://www.rust-lang.org/tools/install)

---

## Setup and Installation

### Building the Elixir Binary

1. Navigate to the Elixir project directory:

   ```bash
   cd ./midway
   ```

2. Build the release:

   ```bash
   MIX_ENV=prod mix release
   ```

### Integrating with Tauri

Once the Elixir build is complete:

1. Run the included script to adjust the generated Elixir build output to match Tauri’s sidecar naming conventions:

    ```bash
    ./adjust_midway.sh
    ```

   This script modifies the `midway.bat` and generates a correctly named and adjusted `midway-{targetTriple}` binary in [midway/_build/prod/rel/midway/bin](midway/_build/prod/rel/midway/bin) from the original `midway` binary. 
   This ensures Tauri automatically includes it during the build process.
   For more details on the required naming conventions, please refer to the [Tauri sidecar documentation](https://v2.tauri.app/develop/sidecar/).

### Launching the Application

Navigate to the `app` subfolder and start the Tauri application in development mode by running:

```bash
cargo tauri dev
```

---

## Notes

- The Elixir sidecar runs on port **8080** and is accessible outside the Tauri environment.
- This build has been tested on Linux, so platform-specific configurations or adjustments may be necessary for macOS or Windows.
- Mobile support has been minimally tested and has not worked so far.