FROM fluent/fluentd:v1.16-2

ARG ENV
ARG AWS_REGINON
ENV FLUENTD_ENV=$ENV
ENV FLUENTD_AWS_REGINON=$AWS_REGINON

USER root
RUN apk add --update --virtual .build-deps sudo build-base ruby-dev
RUN apk add --no-cache geoip geoip-dev libmaxminddb
RUN apk add automake autoconf libtool


RUN fluent-gem install nokogiri
RUN fluent-gem install fluent-plugin-multi-format-parser
RUN fluent-gem install fluent-plugin-geoip
RUN fluent-gem install fluent-plugin-filter_typecast
RUN fluent-gem install fluent-plugin-bunyan

USER fluent

RUN mkdir /fluentd/etc/log

EXPOSE 24224 24224/udp 8888
