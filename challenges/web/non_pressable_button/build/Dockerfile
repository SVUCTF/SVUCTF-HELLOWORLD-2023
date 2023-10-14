FROM ghcr.io/svuctf/base/php:latest

COPY index.php /var/www/html/
COPY startup /var/www/html/startup
RUN chown -R www-data:www-data /var/www/html/

COPY --chmod=500 init.sh /etc/init.sh

CMD ["/etc/init.sh"]