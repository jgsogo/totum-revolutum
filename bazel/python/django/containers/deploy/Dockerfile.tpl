FROM python:3.13-bookworm

LABEL org.opencontainers.image.source=https://github.com/jgsogo/totum-revolutum

RUN apt-get update
RUN apt-get install -y netcat-openbsd

# set environment variables
ENV PYTHONDONTWRITEBYTECODE=1
ENV PYTHONUNBUFFERED=1

# create directory for the app user
RUN mkdir -p /home/%USER%

# create the app user
RUN addgroup --system %GROUP% && adduser --system --group --uid %USER_UID% %USER%

# create the appropriate directories
ENV HOME=/home/%USER%
ENV APP_HOME=/home/%USER%/web
RUN mkdir $APP_HOME
RUN mkdir $APP_HOME/staticfiles
RUN mkdir $APP_HOME/mediafiles
RUN mkdir $APP_HOME/backups

# chown all the files to the app user
RUN chown -R %USER%:%GROUP% $APP_HOME

# change to the app user
USER %USER%
