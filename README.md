# Godot + Rust Template

A robust "batteries-included" template for building Godot 4.6 games using Rust (GDExtension).

This project uses a custom build system written in Rust (`cargo xtask`) to handle the entire pipeline: downloading the engine, compiling code, creating project files, and exporting the final game.

## 📂 Project Structure

```text
├── game/           # The Godot Project
│   ├── bin/        # Compiled GDExtension binaries (auto-generated)
│   └── project.godot
├── rust/           # Rust Source Code
│   ├── src/        # Your Gameplay Crates
│   └── xtask/      # The Build System Automation
└── builds/         # Final Exported Games (auto-created)
```

## 🚀 Getting Started
Prerequisites
- Rust & Cargo: Install Rust
- Git

### Setup

Run the setup command to automatically download Godot 4.6 and the matching Export Templates for your OS. This ensures everyone on the team uses the exact same engine version.

```bash
cargo xtask setup
```

Artifacts are stored in .godot_bin/ (ignored by git).

### Development

To compile the Rust code, copy the libraries to the game project, and open the Godot Editor:

```bash
cargo xtask editor
```

### Play

To compile and immediately launch the game (without the editor):

```bash
cargo xtask run
```

### Release

To build the game in Release mode and export a standalone executable:

```bash
cargo xtask package
```

The output will be found in the builds/ directory.

## 🛠 Automation Features
The xtask system handles the following automations:
- **Version Control**: Downloads the specific Godot version defined in rust/xtask/src/main.rs.
- **OS Compatibility**: Automatically handles paths and permissions for Windows, macOS, and Linux.
- **GDExtension**: Generates the .gdextension configuration file automatically.
- **Bootstrap**: If project.godot or main.tscn are missing, it generates minimal versions so you can start coding immediately.
- **Export**: Generates a default export_presets.cfg if one is missing.

## 📝 License
See LICENSE for details.