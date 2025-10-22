# ReLaunchpad

**The MacOS Launchpad, brought back from the dead.**

> **Status:** Very unstable: icons are broken and functionality may be incomplete. Use at your own risk!

## Overview

ReLaunchpad is a cross-platform desktop application inspired by the original MacOS Launchpad (RIP 2025). Built with [Svelte](https://svelte.dev/), [Rust](https://www.rust-lang.org/), and [Tauri](https://tauri.app/), it aims to provide a fast, native-like app launcher experience.

**Main technologies:**
- Svelte (frontend UI)
- Rust (backend logic)
- Tauri (desktop app wrapper)
- CSS & JavaScript

## Features

- Familiar Launchpad-style grid for your apps
- Lightning-fast performance
- Native integration using Tauri

## Quick Start

### 1. Install prerequisites

- **Node.js** & **npm** (recommended: latest LTS)
- **Rust** & **cargo** ([Install Rust](https://rustup.rs/))
- **Tauri prerequisites:**  
  - macOS: Xcode Command Line Tools (`xcode-select --install`)  
  - [See full Tauri requirements](https://tauri.app/v1/guides/getting-started/prerequisites/)

### 2. Clone the repository

```bash
git clone https://github.com/NellowTCS/ReLaunchpad.git
cd ReLaunchpad
```

### 3. Install & run

```bash
cd Build
npm install
npm run tauri:dev
```

## Contributing

Contributions are welcome!  
Feel free to open issues or pull requests for bug fixes, improvements, or new features.

## License

“Launchpad” and all MacOS trademarks, names, and references are property of Apple Inc.  
This project is not affiliated with, endorsed by, or sponsored by Apple Inc.  
All references are for descriptive purposes only.

[MIT](LICENSE)

## Credits

- Inspired by the old MacOS Launchpad
- Powered by [Svelte](https://svelte.dev/), [Rust](https://www.rust-lang.org/), and [Tauri](https://tauri.app/)
