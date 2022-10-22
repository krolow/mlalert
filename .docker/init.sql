CREATE DATABASE mlalert_test
    WITH
    OWNER = mlalert
    ENCODING = 'UTF8'
    LC_COLLATE = 'en_US.utf8'
    LC_CTYPE = 'en_US.utf8'
    TABLESPACE = pg_default
    CONNECTION LIMIT = -1;
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
