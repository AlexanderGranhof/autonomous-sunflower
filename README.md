# Setup

1. Follow the [espressif esp32 toolchain installation guide](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/get-started/linux-macos-setup.html#step-1-install-prerequisites)
2. Install [rustup](https://rustup.rs/) if its not already installed
3. Install [python3](https://www.python.org/downloads/) if its not already installed
4. Install the [esp32 rust subcommands](https://github.com/esp-rs/esp-idf-template?tab=readme-ov-file#install-cargo-sub-commands) for flashing and building

Now you are able to build and flash an esp32

# Flashing
1. Set the SSID and SSID_PASSWORD in `.cargo/config.toml` env
2. Run `cargo espflash flash --monitor --release --partition-table min_spiffs.csv` to flash the esp32