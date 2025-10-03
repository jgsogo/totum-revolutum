FROM python:3.13-bookworm

LABEL org.opencontainers.image.source=https://github.com/jgsogo/totum-revolutum

RUN apt-get update
RUN apt-get install -y netcat-openbsd libpq

# Install postgresql 17 (not yet available in Debian repos)
RUN apt-get install -y lsb-release && \
    sh -c 'echo "deb http://apt.postgresql.org/pub/repos/apt $(lsb_release -cs)-pgdg main" > /etc/apt/sources.list.d/pgdg.list' && \
    wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | apt-key add - && \
    apt-get update && \
    apt-get install -y postgresql-client

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

# chown all the files to the app user
RUN chown -R %USER%:%GROUP% $APP_HOME

# change to the app user
USER %USER%
