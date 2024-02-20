-- Dev ONLY — Brute Force DROP DB (for local dev and unit test)
SELECT
  pg_terminate_backend(pid)
FROM
  pg_stat_activity
WHERE
  usename = 'app_user'
  OR datname = 'app_db';
DROP DATABASE IF EXISTS app_db;

CREATE USER app_user PASSWORD 'dev_only_pwd';
CREATE DATABASE app_db owner app_user ENCODING = 'UTF-8';
