-- Create the notes table (example resource)
CREATE TABLE IF NOT EXISTS notes (
    id      SERIAL PRIMARY KEY,
    title   TEXT NOT NULL,
    content TEXT NOT NULL DEFAULT '',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Disable RLS so the Data API can read/write freely.
ALTER TABLE notes DISABLE ROW LEVEL SECURITY;
