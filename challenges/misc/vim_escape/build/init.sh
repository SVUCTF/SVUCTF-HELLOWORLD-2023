#!/bin/sh

echo $GZCTF_FLAG > /flag
chmod 444 /flag

unset GZCTF_FLAG

ttyd -W vim /root/.vimrc