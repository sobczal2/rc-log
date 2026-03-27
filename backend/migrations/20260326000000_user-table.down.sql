-- Drop indexes
DROP INDEX IF EXISTS "user".idx_user_username;
DROP INDEX IF EXISTS "user".idx_user_email;

-- Drop tables
DROP TABLE IF EXISTS "user"."user";

-- Drop schema
DROP SCHEMA IF EXISTS "user";
