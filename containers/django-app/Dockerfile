FROM python:3.12-bookworm

RUN apt-get update
RUN apt-get install -y netcat-openbsd

# set environment variables
ENV PYTHONDONTWRITEBYTECODE=1
ENV PYTHONUNBUFFERED=1

# create directory for the app user
RUN mkdir -p /home/app

# create the app user
RUN addgroup --system app && adduser --system --group --uid 1234 app

# create the appropriate directories
ENV HOME=/home/app
ENV APP_HOME=/home/app/web
RUN mkdir $APP_HOME
RUN mkdir $APP_HOME/staticfiles
RUN mkdir $APP_HOME/mediafiles

# chown all the files to the app user
RUN chown -R app:app $APP_HOME

# change to the app user
USER app

# WORKDIR $APP_HOME
