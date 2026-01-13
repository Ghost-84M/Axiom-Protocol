This folder contains helper files to run the Qubit node as a persistent service.

Files:
- qubit.service: systemd unit template (adjust WorkingDirectory and ExecStart before installing).
- run_qubit.sh: lightweight supervisor script to restart the node if it exits. Use this when systemd is not available.
- qubit.logrotate: logrotate config template for `node.log` (replace the path before installing).

Quick usage (systemd available and running):

1. Adjust `contrib/qubit.service` paths.
2. Install and enable:

```sh
sudo cp contrib/qubit.service /etc/systemd/system/qubit.service
sudo systemctl daemon-reload
sudo systemctl enable --now qubit.service
sudo journalctl -u qubit.service -f
```

Quick usage (no systemd):

1. Build the release: `cargo build --release`
2. Start the supervisor in background:

```sh
nohup contrib/run_qubit.sh &> /dev/null &
```

Quick install for logrotate (optional):

```sh
# Edit contrib/qubit.logrotate and replace /path/to/... with the repository path
sudo cp contrib/qubit.logrotate /etc/logrotate.d/qubit
sudo logrotate -f /etc/logrotate.d/qubit
```
