#!/bin/sh

echo $GZCTF_FLAG > /flag
unset GZCTF_FLAG

/usr/bin/vim -u /root/.vimrc /root/.vimrc