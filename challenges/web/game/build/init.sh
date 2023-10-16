#!/bin/sh

flag="$GZCTF_FLAG"
reversed_flag=$(echo "$flag" | rev)

sed -i "s/%FLAG%/$reversed_flag/g" /var/www/html/src/game.js

php-fpm -D
nginx -g 'daemon off;'