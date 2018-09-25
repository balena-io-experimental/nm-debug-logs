#!/bin/bash

LOGFILE='/mnt/data/resin-data/nm-debug-logs.txt'

echo "===== Starting collecting logs $(date +'%F %T')..."

while true
do
    echo 'Collecting debug logs...'

    journalctl --since "$(date --date='-1 minute' +'%F %T')" \
        | tail -n +2 >> $LOGFILE

    sleep 1m
done
