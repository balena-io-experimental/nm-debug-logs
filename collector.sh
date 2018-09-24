#!/bin/bash

LOGFILE='/mnt/data/resin-data/nm-debug-logs.txt'

dbus-send --system --dest=fi.w1.wpa_supplicant1 \
    /fi/w1/wpa_supplicant1 \
    org.freedesktop.DBus.Properties.Set \
    string:fi.w1.wpa_supplicant1 string:DebugTimestamp variant:boolean:true

dbus-send --system --dest=fi.w1.wpa_supplicant1 \
    /fi/w1/wpa_supplicant1 \
    org.freedesktop.DBus.Properties.Set \
    string:fi.w1.wpa_supplicant1 string:DebugLevel variant:string:"debug"

dbus-send --system --dest=org.freedesktop.NetworkManager \
    /org/freedesktop/NetworkManager \
    org.freedesktop.NetworkManager.SetLogging \
    string:"debug" string:""

echo "===== Starting collecting logs $(date +'%F %T')..." >> $LOGFILE

while true
do
    echo 'Collecting debug logs...'

    journalctl --since "$(date --date='-1 minute' +'%F %T')" \
        | tail -n +2 >> $LOGFILE

    sleep 1m
done
