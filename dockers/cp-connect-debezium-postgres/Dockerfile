FROM confluentinc/cp-server-connect-base:7.5.0

ENV CONNECT_PLUGIN_PATH="/usr/share/java,/usr/share/confluent-hub-components"

RUN confluent-hub install --no-prompt debezium/debezium-connector-postgresql:2.2.1