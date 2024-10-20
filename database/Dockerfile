FROM postgres:17.0

# Copy sql scripts
COPY ./tables.sql /docker-entrypoint-initdb.d/10-tables.sql
COPY ./helpers.sql /docker-entrypoint-initdb.d/20-helpers.sql
COPY ./grants_inserts.sql /docker-entrypoint-initdb.d/30-grants_inserts.sql
COPY ./roles_inserts.sql /docker-entrypoint-initdb.d/40-roles_inserts.sql
COPY ./views.sql /docker-entrypoint-initdb.d/50-views.sql

COPY ./inserts.sql /docker-entrypoint-initdb.d/99-inserts.sql
