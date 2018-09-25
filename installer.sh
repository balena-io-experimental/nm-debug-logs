#!/bin/bash

set -u

trap "exit 1" TERM
export TOP_PID=$$

APP='NetworkManager Debug Logs Collector'

SCRIPT='collector.sh'
SERVICE='collector.service'
LOGGING='logging.conf'

BASE_URL='https://github.com/resin-io-playground/nm-debug-logs/raw/master'
SCRIPT_URL="$BASE_URL/$SCRIPT"
SERVICE_URL="$BASE_URL/$SERVICE"
LOGGING_URL="$BASE_URL/$LOGGING"

INSTALLER="$APP Installer"

INSTALL=true

usage() {
    cat 1>&2 <<EOF
$INSTALLER 1.0.0 (2018-24-09)

FLAGS:
    -u, --uninstall         Uninstall
    -h, --help              Prints help information
EOF
}

main() {
    for arg in "$@"; do
        case "$arg" in
            -h|--help)
                usage
                exit 0
                ;;
            -u|--uninstall)
                INSTALL=false
                ;;
            *)
                ;;
        esac
    done

    if [ "$INSTALL" = true ]; then
        install_collector
    else
        uninstall_collector
    fi
}

install_collector() {
    say 'Remounting root filesystem as read/write.'

    ensure mount -o remount,rw /

    say "Downloading and installing $SCRIPT_URL..."

    ensure mkdir -p /usr/local/sbin
    ensure curl -Lsf "$SCRIPT_URL" -o "/usr/local/sbin/$SCRIPT"
    ensure chmod +x "/usr/local/sbin/$SCRIPT"

    say "Downloading and installing $SERVICE_URL..."

    ensure curl -Lsf "$SERVICE_URL" -o "/etc/systemd/system/$SERVICE"

    say "Downloading and installing $LOGGING_URL..."

    ensure curl -Lsf "$LOGGING_URL" -o "/etc/NetworkManager/conf.d/$LOGGING"

    say 'Restarting NetworkManager'

    ensure systemctl daemon-reload

    ensure systemctl restart NetworkManager.service

    say "Starting $SERVICE..."

    ensure systemctl enable collector.service

    ensure systemctl restart collector.service

    say 'Remounting root filesystem as read only.'

    ensure mount -o remount,ro /

    say "Successfully installed $APP."
}

uninstall_collector() {
    say "Starting $SERVICE..."

    ensure systemctl stop collector.service

    say 'Remounting root filesystem as read/write.'

    ensure mount -o remount,rw /

    say "Deleting $SERVICE..."

    rm "/usr/local/sbin/$SCRIPT"

    rm "/etc/systemd/system/$SERVICE"

    say 'Deleting NetworkManager logging level config'

    rm "/etc/NetworkManager/conf.d/$LOGGING"

    say 'Remounting root filesystem as read only.'

    ensure mount -o remount,ro /

    say 'Restarting NetworkManager'

    ensure systemctl daemon-reload

    ensure systemctl restart NetworkManager.service

    ensure systemctl restart wpa_supplicant.service

    say "Successfully uninstalled $APP."
}

say() {
    printf '\33[1mInstaller:\33[0m %s\n' "$1"
}

err() {
    printf '\33[1;31mInstaller:\33[0m %s\n' "$1" >&2
    kill -s TERM $TOP_PID
}

ensure() {
    "$@"
    if [ $? != 0 ]; then
        err "command failed: $*";
    fi
}

main "$@" || exit 1
