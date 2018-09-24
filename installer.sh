#!/bin/bash

set -u

trap "exit 1" TERM
export TOP_PID=$$

APP='NetworkManager Debug Logs Collector'

SCRIPT='collector.sh'
SERVICE='collector.service'

BASE_URL='https://github.com/resin-io-playground/nm-debug-logs/raw/master'
SCRIPT_URL="$BASE_URL/$SCRIPT"
SERVICE_URL="$BASE_URL/$SERVICE"

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

    say 'Remounting root filesystem as read only.'

    ensure mount -o remount,ro /

    say "Starting $SERVICE..."

    ensure systemctl daemon-reload

    ensure systemctl restart collector.service

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

    say 'Remounting root filesystem as read only.'

    ensure mount -o remount,ro /

    ensure systemctl daemon-reload

    say 'Setting NetworkManager/wpa_supplicant logging level back to info...'

    dbus-send --system --dest=fi.w1.wpa_supplicant1 \
        /fi/w1/wpa_supplicant1 \
        org.freedesktop.DBus.Properties.Set \
        string:fi.w1.wpa_supplicant1 string:DebugLevel variant:string:"info"

    dbus-send --system --dest=org.freedesktop.NetworkManager \
        /org/freedesktop/NetworkManager \
        org.freedesktop.NetworkManager.SetLogging \
        string:"info" string:""

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
