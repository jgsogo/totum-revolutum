FROM nginx:1.29

LABEL org.opencontainers.image.source=https://github.com/jgsogo/totum-revolutum

RUN rm /etc/nginx/conf.d/default.conf
COPY nginx.conf /etc/nginx/conf.d
