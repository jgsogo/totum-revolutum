FROM nginx:1.31

LABEL org.opencontainers.image.source=https://github.com/jgsogo/totum-revolutum

RUN rm /etc/nginx/conf.d/default.conf
COPY nginx.conf /etc/nginx/conf.d
