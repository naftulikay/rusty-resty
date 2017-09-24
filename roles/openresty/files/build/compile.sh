#!/bin/bash

DEBUG=n
OPENRESTY_VERSION="${OPENRESTY_VERSION:-1.11.2.5}"
NCHAN_VERSION="${NCHAN_VERSION:-1.1.7}"

function .is-debug() {
  test "$DEBUG" == "y"
}

function .log() {
  echo '***' $@ >&2
}

function .compile() {

  (
    set -e
    cd /usr/local/src/openresty-${OPENRESTY_VERSION}

    .log "Configuring OpenResty..."

    ./configure \
      --with-luajit \
      --with-stream \
      --with-threads \
      --with-pcre-jit \
      --with-http_v2_module \
      --with-http_realip_module \
      --with-http_stub_status_module \
      --with-http_ssl_module \
      --with-http_gzip_static_module \
      --with-http_sub_module \
      --with-http_secure_link_module \
      --with-http_addition_module \
      --with-http_auth_request_module \
      --with-stream_ssl_module \
      --with-http_flv_module \
      --with-http_mp4_module \
      --with-http_gunzip_module \
      --with-http_geoip_module \
      --with-http_iconv_module \
      --with-http_image_filter_module \
      --with-http_postgres_module \
      --with-mail=dynamic \
      --with-http_perl_module=dynamic \
      --add-dynamic-module=/usr/local/src/nchan-${NCHAN_VERSION} \
      --prefix=/usr/share/openresty \
      --user=nginx \
      --group=nginx \
      --sbin-path=/usr/sbin/nginx \
      --pid-path=/var/run/nginx.pid \
      --lock-path=/var/lock/nginx.lock \
      --conf-path=/etc/nginx/nginx.conf \
      --http-log-path=/var/log/nginx/access.log \
      --error-log-path=/var/log/nginx/error.log \
      --http-proxy-temp-path=/var/lib/nginx/proxy \
      --http-fastcgi-temp-path=/var/lib/nginx/fastcgi \
      --http-client-body-temp-path=/var/lib/nginx/body \
      --http-uwsgi-temp-path=/var/lib/nginx/uwsgi \
      --http-scgi-temp-path=/var/lib/nginx/scgi

    .log "Building OpenResty..."
    make

    .log "Installing OpenResty..."
    make install
  )
}

function .main() {
  .compile
}

if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
  set -e
  .main $@
fi
