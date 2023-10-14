#!/bin/sh

echo $GZCTF_FLAG > /home/ctf/flag
chmod 400 /home/ctf/flag
chown -R root:root /home/ctf/flag
unset GZCTF_FLAG

/usr/sbin/chroot --userspec=ctf /home/ctf/ /bin/sh