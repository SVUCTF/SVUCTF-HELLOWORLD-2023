#!/bin/sh

echo $GZCTF_FLAG > /flag
chmod 444 /flag

/app/oj --config /app/config.json
