#!/bin/sh

echo $GZCTF_FLAG > /home/ctf/flag
chmod 400 /home/ctf/flag
chown -R root:root /home/ctf/flag
unset GZCTF_FLAG

su ctf
/usr/sbin/chroot /home/ctf/ /bin/sh