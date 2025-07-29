# Audio controller

## Setup

### Windows with WSL

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

#### Flash COM permissions

To allow WSL environment to flash directy

1. ✅ Install usbipd-win (if not already)

🧪 Solution Options
🥇 Option 1: Use usbipd to forward COM port to WSL2 (Best)
This lets your WSL2 instance talk to USB devices like serial ports.

Step-by-step:
1. ✅ Install usbipd-win (if not already)
``` Powershell
winget install usbipd
```
2. ✅ List USB devices on Windows (PowerShell, not WSL)
```
usbipd list
``` Powershell
Look for your ESP32 device (likely shows as "USB Serial Device" or similar).

You’ll see an output like:

arduino
Copy
BUSID  VID:PID    DEVICE
2-2    10C4:EA60  USB Serial Device (COM4)
Take note of the BUSID (e.g., 2-2).

3. ✅ Attach USB device to WSL (PowerShell):
powershell
Copy
usbipd wsl attach --busid 2-2
You should see:

arduino
Copy
USB device attached to WSL
4. ✅ Inside WSL, confirm device shows up:
bash
Copy
ls /dev/ttyUSB*
Expected: /dev/ttyUSB0 or similar.

🔁 Now you can flash directly from WSL:
bash
Copy
espflash /dev/ttyUSB0 --monitor
🎉 Done! No need to copy .bin to Windows anymore.

🔁 Optional: Add udev rule for permissions (if needed)
If you get a Permission denied when accessing /dev/ttyUSB0, run this:

bash
Copy
sudo usermod -a -G dialout $USER
Then restart WSL or log out and back in.

To generate this project

```
sudo apt update
sudo apt install pkg-config libssl-dev -y
cargo install cargo-generate
```

Setup flashing

- Setup espflash
    ```
    cargo install espflash
    export PATH="$HOME/.cargo/bin:$PATH"
    echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
    ```
- Restart terminal
- Allow port connection from WSL `winget install usbipd`


Flash

Run `wsl --list --verbose`
Run `usbipd bind --busid 2-1`
Run `usbipd attach --busid 2-1 --wsl`
Run `sudo usermod -aG dialout $USER` to add your user to the `dailout` group

- Hold the BOOT button
- While holding BOOT, tap the RESET button
- Release BOOT

# Full Flashing steps

1. Run `cargo clean`

2. Run `cargo build --release`
This generates at `target/xtensa-esp32s3-espidf/release/audio-controller`

3. Copy bin from WSL to windows

``` WSL
cp target/xtensa-esp32s3-espidf/release/audio-controller /mnt/c/Users/camer/Desktop/audio-controller.bin
```

4. Check COM port
In windows Powershell Run:
```
Get-PnpDevice -PresentOnly | Where-Object { $_.Class -eq 'Ports' }
```

The output should look like

```
Status     Class           FriendlyName                                                                     InstanceId
------     -----           ------------                                                                     ----------
OK         Ports           USB-Enhanced-SERIAL CH343 (COM5)                                                 USB\VID_...
```

5. Flash from windows
As administrator run from windows powershell:
```
espflash flash --port COM5 C:\Users\camer\Desktop\audio-controller.bin --monitor
```



#Flashing using a script

Dialout the COMs

``` Powershell
wsl -d Ubuntu -u root
```

This marks the script as executable.

```
chmod +x ./scripts/flash.sh
```