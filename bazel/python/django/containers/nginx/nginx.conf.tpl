upstream django_application {
    server web:%DJANGO_PORT%;
}

server {

    listen 80;

    location /static/ {
        alias /home/%USER%/web/staticfiles/;
    }

    location /media/ {
        alias /home/%USER%/web/mediafiles/;
    }

    location / {
        proxy_pass http://django_application;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header Host $http_host;
        proxy_redirect off;
    }

}
