-- Add migration script here
-- Drop existing table and types (if they exist)
DROP TABLE IF EXISTS users CASCADE;
DROP TYPE IF EXISTS user_role CASCADE;
DROP TYPE IF EXISTS auth_provider CASCADE;

-- Recreate with correct enum values
CREATE TYPE user_role AS ENUM ('Admin', 'User', 'Guest');
CREATE TYPE auth_provider AS ENUM ('Local', 'Google');

-- Create the users table
CREATE TABLE users (
    id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
    name VARCHAR(100) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    verified BOOLEAN NOT NULL DEFAULT FALSE,
    password VARCHAR(100) NOT NULL,
    verification_token VARCHAR(255),
    token_expires_at TIMESTAMP WITH TIME ZONE,
    role user_role NOT NULL DEFAULT 'Guest',
    provider auth_provider NOT NULL DEFAULT 'Local',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create index
CREATE INDEX users_email_idx ON users (email);