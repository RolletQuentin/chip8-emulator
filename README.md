# Rust Chip8 Emulator

Welcome to my Chip8 emulator! This project is an implementation based on a tutorial from [developpez.com](https://jeux.developpez.com/tutoriels/programmer-emulateur-console/tutoriel-1-tour-d-horizon/) (in French).

## Quick Start

Make sure you have Rust installed on your computer. This emulator has been tested only on the Wayland environment. It doesn't implement sounds yet.

```shell
git clone https://github.com/RolletQuentin/chip8-emulator
cd chip8-emulator
cargo build --release
./target/release/chip8-emulator <rom_path>
```

You can find several ROMs included in the repository to test the emulator. For more games, you can visit the [chip8-roms](https://github.com/kripod/chip8-roms) repository.
