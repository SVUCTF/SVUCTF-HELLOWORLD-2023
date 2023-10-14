#!/bin/sh

echo $GZCTF_FLAG > /home/ctf/flag
chown -R root:root /home/ctf/flag
unset GZCTF_FLAG

/usr/sbin/chroot /home/ctf/ /bin/sh