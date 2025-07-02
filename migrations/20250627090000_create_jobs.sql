-- Create job_type enum if not exists
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'job_type') THEN
        CREATE TYPE job_type AS ENUM ('Remote', 'OnSite', 'Hybrid');
    END IF;
END$$;

-- Create round_categories table
CREATE TABLE round_categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(50) NOT NULL UNIQUE
);

-- Seed default round categories
INSERT INTO round_categories (name) VALUES
    ('Aptitude'),
    ('Technical'),
    ('HR'),
    ('Managerial'),
    ('GroupDiscussion'),
    ('Coding'),
    ('Other');

-- Create jobs table
CREATE TABLE jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    company VARCHAR(255) NOT NULL,
    location VARCHAR(255) NOT NULL,
    salary_min INTEGER,
    salary_max INTEGER,
    job_type job_type NOT NULL,
    rounds INTEGER NOT NULL,
    round_details JSONB,
    experience_min INTEGER,
    experience_max INTEGER,
    is_remote BOOLEAN DEFAULT FALSE,
    application_deadline DATE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create job_skills join table
CREATE TABLE job_skills (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    job_id UUID REFERENCES jobs(id) ON DELETE CASCADE,
    skill_id UUID REFERENCES skills(id) ON DELETE CASCADE
);