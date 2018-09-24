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

UNINSTALL=false

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
                UNINSTALL=true
                ;;
            *)
                ;;
        esac
    done

    if [ "$UNINSTALL" = false ]; then
        install_collector
    else
        say "Uninstalled."
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

    ensure mount -o remount,ro /

    say "Starting $SERVICE..."

    ensure systemctl daemon-reload

    ensure systemctl restart collector.service

    say "Successfully installed $APP."
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
