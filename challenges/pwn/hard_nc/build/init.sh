#!/bin/sh

echo $GZCTF_FLAG > /home/ctf/.secret/flag
chown -R ctf:ctf /home/ctf/.secret/flag
unset GZCTF_FLAG

/usr/sbin/chroot /home/ctf/ /bin/sh
