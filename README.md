## Resin OS NetworkManager Debug Logs Collector

For debugging networking issues it is often needed to enable verbose logging level of NetworkManager. On Resin OS the number of log entries is limited by the size of the state partition. This is insufficient when NetworkManager log levels are set to verbose. The provided script collects and stores the system logs in the data partition of the Resin device.

_NOTE: Use the provided service script only when debugging networking issues as the collected log entries will fill up your data partition relatively quickly over time._ 

## Install

To install the Resin OS NetworkManager Debug Logs Collector run the following from a host OS terminal:

`curl https://github.com/resin-io-playground/nm-debug-logs/raw/master/installer.sh -sSfL | bash`

The installer script will install a new systemd logging service script that collects and stores log entries in `/mnt/data/resin-data/nm-debug-logs.txt`. The service script persists across device reboots, however it will not persist across host OS updates.

## Uninstall

To uninstall and revert to the original state run:

`curl https://github.com/resin-io-playground/nm-debug-logs/raw/master/installer.sh -sSfL | bash -s -- --uninstall`
