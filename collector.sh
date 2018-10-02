#!/bin/bash

LOGFILE='/mnt/data/resin-data/nm-debug-logs.txt'

echo "===== Starting collecting logs $(date +'%F %T')..."

journalctl -b >> $LOGFILE

while true
do
    sleep 1m

    echo 'Collecting debug logs...'

    journalctl --since "$(date --date='-1 minute' +'%F %T')" \
        | tail -n +2 >> $LOGFILE
done
