# Safe Shutdown

## Installation

### Create a systemd user service

To create a service that requires no root permissions, it has to be located in `$HOME/.config/systemd/user`

In this, the service file for `safe_shutdown` will be located in `$HOME/.config/systemd/user/safe_shutdown.service`

Copy the content below into the service file. Amend as you see fit

```
[Unit]
Description=Safe shutdown Service
After=network.target
[Service]
Type=simple
WorkingDirectory=/home/<your-user-name>/.cargo/bin/
ExecStart=/home/<your-user-name>/.cargo/bin/safe-shutdown
Restart=on-failure
[Install]
WantedBy=default.target
```

### Install from git

Run the below command to install `safe_shutdown`. The installation path will default to `/home/<your-user-name>/.cargo`

Consult `cargo install` documentation for details

If the installation path is not `/home/<your-user-name>/.cargo`, you will have to amend the service file to the path where the binary is located

```sh
cargo install --git https://github.com/daviddexter/safe_shutdown.git --tag v0.0.1
```

### Controlling the service

```sh
# Control whether service loads on boot
systemctl --user enable safe_shutdown.service
systemctl --user disable safe_shutdown.service
# Manual start and stop
systemctl --user start safe_shutdown.service
systemctl --user stop safe_shutdown.service
# Restarting/reloading
systemctl --user daemon-reload  # Run if safe_shutdown.service file has changed
systemctl --user restart safe_shutdown.service
```
