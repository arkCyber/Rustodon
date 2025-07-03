-- Remove fields from lists table
ALTER TABLE lists DROP COLUMN IF EXISTS title;
ALTER TABLE lists DROP COLUMN IF EXISTS replies_policy;
ALTER TABLE lists DROP COLUMN IF EXISTS exclusive;
