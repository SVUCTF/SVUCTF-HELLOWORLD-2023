#!/bin/sh

echo $GZCTF_FLAG > /flag
chmod 444 /flag

unset GZCTF_FLAG

/app/oj --config /app/config.json