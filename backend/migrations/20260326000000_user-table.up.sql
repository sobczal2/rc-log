-- Create user schema
CREATE SCHEMA IF NOT EXISTS "user";

-- Create user table
CREATE TABLE "user"."user" (
    id UUID PRIMARY KEY,
    username VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL
);

-- Create indexes
CREATE INDEX idx_user_username ON "user"."user"(username);
CREATE INDEX idx_user_email ON "user"."user"(email);
