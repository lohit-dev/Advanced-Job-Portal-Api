-- Create auth_provider enum type
CREATE TYPE auth_provider AS ENUM ('Local', 'Google');

-- Add provider column to users table
ALTER TABLE users 
ADD COLUMN provider auth_provider NOT NULL DEFAULT 'Local';

-- Update existing users to have Local provider
UPDATE users SET provider = 'Local' WHERE provider IS NULL; 