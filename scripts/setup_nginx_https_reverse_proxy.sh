#!/usr/bin/env bash
set -euo pipefail
DOMAIN=${1:-arozos.local}
UPSTREAM=${2:-127.0.0.1:8080}
CERT=${3:-/etc/letsencrypt/live/$DOMAIN/fullchain.pem}
KEY=${4:-/etc/letsencrypt/live/$DOMAIN/privkey.pem}

if ! command -v nginx >/dev/null 2>&1; then
  echo "nginx wurde nicht gefunden. Installiere es zuerst:" >&2
  echo "  Debian/Ubuntu: sudo apt install nginx" >&2
  echo "  Arch:          sudo pacman -S nginx" >&2
  echo "  Alpine:        sudo apk add nginx" >&2
  echo "  macOS:         brew install nginx" >&2
  exit 1
fi

if [ -d /etc/nginx/sites-available ]; then
  CONF=/etc/nginx/sites-available/arozos-rust.conf
  ENABLE_LINK=/etc/nginx/sites-enabled/arozos-rust.conf
  sudo mkdir -p /etc/nginx/sites-available /etc/nginx/sites-enabled
elif [ -d /etc/nginx/conf.d ]; then
  CONF=/etc/nginx/conf.d/arozos-rust.conf
  ENABLE_LINK=""
else
  CONF=/etc/nginx/arozos-rust.conf
  ENABLE_LINK=""
  sudo mkdir -p /etc/nginx
fi

cat <<NGINX | sudo tee "$CONF" >/dev/null
server {
    listen 80;
    server_name $DOMAIN;
    return 301 https://\$host\$request_uri;
}

server {
    listen 443 ssl http2;
    server_name $DOMAIN;

    ssl_certificate $CERT;
    ssl_certificate_key $KEY;
    client_max_body_size 0;

    proxy_read_timeout 3600s;
    proxy_send_timeout 3600s;

    location / {
        proxy_pass http://$UPSTREAM;
        proxy_http_version 1.1;
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto https;
        proxy_set_header Upgrade \$http_upgrade;
        proxy_set_header Connection "upgrade";
    }

    location /webdav/ {
        proxy_pass http://$UPSTREAM/webdav/;
        proxy_http_version 1.1;
        proxy_request_buffering off;
        proxy_buffering off;
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto https;
        proxy_set_header Destination \$http_destination;
        proxy_set_header Overwrite \$http_overwrite;
        proxy_set_header Depth \$http_depth;
        proxy_set_header Authorization \$http_authorization;
    }
}
NGINX

if [ -n "$ENABLE_LINK" ]; then
  sudo ln -sf "$CONF" "$ENABLE_LINK"
fi

echo "Geschrieben: $CONF"
sudo nginx -t
if command -v systemctl >/dev/null 2>&1; then
  sudo systemctl reload nginx || sudo systemctl restart nginx
else
  sudo service nginx reload || sudo nginx -s reload
fi
