# Audio controller

## Setup

### Windows

- Install wsl2
- Install wsl2 distrobution (Recommended Ubuntu)
- Install rustup
    - Run `source $HOME/.cargo/env` to setup the cargo commands
- Install C compiler
    - Run `sudo apt update`
    - Run `sudo apt install build-essential -y`
- Run `cargo install espup --locked`
- Run `espup install`
- activate the environment that espup prepared, run `source ~/export-esp.sh`
    - To make this automatic, add it to your .bashrc: `echo 'source ~/export-esp.sh' >> ~/.bashrc`
- For python - Run`cargo install ldproxy`
- Make sure your environment is ready: `source ~/export-esp.sh`
- Install python since ESP-IDF requires it to build `sudo apt install python3.12-venv -y`





To generate this project

```
sudo apt update
sudo apt install pkg-config libssl-dev -y
cargo install cargo-generate
```