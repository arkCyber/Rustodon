-- Up migration
ALTER TABLE filter_keywords ADD COLUMN IF NOT EXISTS whole_word BOOLEAN DEFAULT FALSE;
